use super::{Location, SaveError};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    io::{Error as IOError, ErrorKind},
    path::Path,
};

#[cfg(all(feature = "stdweb", feature = "web_sys"))]
compile_error!("stdweb and web_sys may not both be enabled at once, please pick one");

#[cfg(all(not(feature = "stdweb"), not(feature = "web_sys")))]
compile_error!("Please enable one of stdweb or web_sys to compile for wasm");

#[cfg(feature = "stdweb")]
#[path = "web/stdweb.rs"]
mod backend;

#[cfg(feature = "web_sys")]
#[path = "web/web_sys.rs"]
mod backend;

pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    let path = path.as_ref().to_str().expect("The path must be able to be stringified");

    backend::make_request(path)
}

pub fn save<T: Serialize>(location: Location, _appname: &str, profile: &str, data: &T)
        -> Result<(), SaveError> {
    backend::set_storage(
        location == Location::Cache,
        profile,
        serde_json::to_string(data)?.as_str()
    )
}

pub fn save_raw(location: Location, _appname: &str, profile: &str, data: &[u8])
    -> Result<(), SaveError> {
    backend::set_storage(
        location == Location::Cache,
        profile,
        base64::encode(data).as_str()
    )
}

pub fn load<T>(location: Location, _appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    let value = backend::get_storage(location == Location::Cache, profile)?;

    Ok(serde_json::from_str(value.as_str())?)
}

pub fn load_raw(location: Location, _appname: &str, profile: &str) -> Result<Vec<u8>, SaveError> {
    let value = backend::get_storage(location == Location::Cache, profile)?;

    base64::decode(value.as_str()).map_err(|_| SaveError::DecodeError)
}

fn web_try<T, E>(result: Result<T, E>, error: &str) -> Result<T, IOError> {
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(new_wasm_error(error))
    }
}

fn new_wasm_error(string: &str) -> IOError {
    IOError::new(ErrorKind::NotFound, string)
}
