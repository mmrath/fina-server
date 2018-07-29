#![feature(extern_prelude, const_fn)]
#![feature(plugin, decl_macro, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate fina_app_lib;
extern crate fina_model as model;
extern crate fina_util as util;
extern crate parking_lot;
extern crate reqwest;
extern crate rocket;

extern crate diesel;
extern crate serde;
extern crate serde_json;

mod common;
mod core;
