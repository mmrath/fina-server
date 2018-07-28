#![feature(extern_prelude)]

extern crate chrono;
extern crate diesel;
#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate fina_model as model;
#[macro_use]
extern crate fina_util as util;

pub mod core;
