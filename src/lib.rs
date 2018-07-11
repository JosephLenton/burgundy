#![allow(missing_docs)]
#![warn(unreachable_pub)]
#![deny(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces)]
#![feature(nll)]
#![feature(try_trait)]
#![feature(box_patterns)]
#![feature(extern_in_paths)]
#![feature(non_modrs_mods)]
#![feature(pattern)]
#![feature(iterator_find_map)]
#![feature(crate_visibility_modifier)]

#[macro_use]
extern crate failure;

mod request_information;

mod burgandy_to_hyper;
mod method;

mod path;
pub use path::Path;

mod domain;
pub use domain::Domain;

mod error;
pub use error::Error;
