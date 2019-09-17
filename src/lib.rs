//! A simple utility to unify loading files on desktop and web
//!
//! On desktop, `load_file` is backed by native file system APIs.
//! On web, it is backed by an HTTP 'GET' request.

#![deny(
    bare_trait_objects,
    missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

use std::{future::Future, io::Error as IOError, path::Path};

/// Create a Future that loads a file into an owned Vec of bytes
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading
pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    platform::load_file(path)
}

// Select which platform implementation to use based on provided features

#[cfg(not(target_arch = "wasm32"))]
#[path = "desktop.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod platform;
