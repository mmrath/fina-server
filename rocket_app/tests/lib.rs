#![feature(extern_prelude, const_fn)]

#![feature(plugin, decl_macro, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate fina_app_lib;
extern crate parking_lot;
extern crate reqwest;
extern crate rocket;

mod common;
mod core;
