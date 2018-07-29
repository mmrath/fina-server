#![feature(rust_2018_preview)]
#![deny(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]


#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate fina_util;


pub mod core;
pub mod error;
pub mod schema;
