#![feature(rust_2018_preview)]
#![deny(rust_2018_compatibility)]
extern crate chrono;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate fina_util as util;

pub mod core;
pub mod error;
pub mod schema;
