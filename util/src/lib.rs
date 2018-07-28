#![feature(rust_2018_preview)]
#![deny(rust_2018_compatibility)]


extern crate diesel;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;
extern crate argonautica;
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate ring;
extern crate serde;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
extern crate erased_serde;
extern crate http;

pub mod macros;

mod context;
pub mod db;
mod util_functions;

pub mod error;

pub use self::context::Context;
pub use self::util_functions::*;
