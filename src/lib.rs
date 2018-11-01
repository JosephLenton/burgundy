#![allow(missing_docs)]
#![warn(unreachable_pub)]
#![deny(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces)]

#[macro_use]
extern crate failure;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio;

mod native_client;
mod request_information;

mod method;
pub use method::Method;

mod path;
pub use path::Path;

mod domain;
pub use domain::Domain;

mod error;
pub use error::Error;
pub use error::Result;

mod response;
pub use response::Response;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;
