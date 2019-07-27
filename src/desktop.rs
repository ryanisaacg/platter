use std::{
    fs::File,
    future::Future,
    io::{Error as IOError, Read},
    path::Path,
};

pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    futures::future::ready(load_data(path))
}

fn load_data(path: impl AsRef<Path>) -> Result<Vec<u8>, IOError> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;

    Ok(data)
}
