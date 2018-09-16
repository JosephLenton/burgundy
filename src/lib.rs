#![allow(missing_docs)]
#![warn(unreachable_pub)]
#![deny(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces)]
#![feature(nll)]
#![feature(try_trait)]
#![feature(box_patterns)]
#![feature(extern_in_paths)]
#![feature(pattern)]
#![feature(crate_visibility_modifier)]

#[macro_use]
extern crate failure;

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
