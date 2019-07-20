use futures::future::{TryFutureExt, ready, poll_fn};
use std::{
    future::Future,
    io::{Error as IOError, ErrorKind},
    task::Poll,
};
use stdweb::{
    Reference,
    InstanceOf,
    unstable::TryInto,
    web::{XmlHttpRequest, ArrayBuffer, TypedArray, XhrReadyState, XhrResponseType, window},
};
use super::SaveError;

pub fn make_request(path: &str) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    ready(create_request(path))
        .and_then(|xhr| poll_fn(move |_| poll_request(&xhr)))
}

fn create_request(path: &str) -> Result<XmlHttpRequest, IOError> {
    let xhr = XmlHttpRequest::new();
    web_try(xhr.open("GET", path), "Failed to create a GET request")?;
    web_try(xhr.send(), "Failed to send a GET request")?;
    web_try(xhr.send(), "Failed to send a GET request")?;
    web_try(xhr.set_response_type(XhrResponseType::ArrayBuffer), "Failed to set the response type")?;
    Ok(xhr)
}

fn poll_request(xhr: &XmlHttpRequest) -> Poll<Result<Vec<u8>, IOError>> {
    let status = xhr.status();
    let ready_state = xhr.ready_state();
    match (status / 100, ready_state) {
        (2, XhrReadyState::Done) => {
            let response: Reference = xhr.raw_response().try_into().expect("The response will always be a JS object");

            let array = if TypedArray::<u8>::instance_of(&response) {
                response.downcast::<TypedArray<u8>>().map(|arr| arr.to_vec())
            } else if ArrayBuffer::instance_of(&response) {
                response.downcast::<ArrayBuffer>().map(|arr| TypedArray::<u8>::from(arr).to_vec())
            } else {
                return Poll::Ready(Err(new_wasm_error(&format!("Unknown file encoding type: {:?}", response))));
            };

            Poll::Ready(if let Some(array) = array {
                Ok(array)
            } else {
                Err(new_wasm_error("Failed to cast file into bytes"))
            })
        },
        (2, _) => Poll::Pending,
        (0, _) => Poll::Pending,
        _ => Poll::Ready(Err(new_wasm_error("Non-200 status code returned")))
    }

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

pub fn set_storage(is_local: bool, profile: &str, value: &str) -> Result<(), SaveError> {
    let storage = if is_local {
        window().local_storage()
    } else {
        window().session_storage()
    };
    
    storage.insert(profile, value).map_err(|_| SaveError::SaveWriteFailed)
}

pub fn get_storage(is_local: bool, profile: &str) -> Result<String, SaveError> {
    let storage = if is_local {
        window().local_storage()
    } else {
        window().session_storage()
    };
    
    storage.get(profile).ok_or_else(|| SaveError::SaveNotFound(profile.to_string()))
}
