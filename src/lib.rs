#![allow(missing_docs)]
#![warn(unreachable_pub)]
#![deny(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces)]

#[allow(unused_imports)]
#[macro_use]
extern crate burgundy_derive;

#[doc(hidden)]
pub use burgundy_derive::*;

extern crate burgundy_lib;
pub use burgundy_lib::*;