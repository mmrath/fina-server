#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
#![feature(rust_2018_preview)]
#![deny(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;

extern crate fina_model as model;
extern crate fina_service as service;
extern crate fina_util as util;
extern crate serde;

mod common;
mod core;

use rocket::Rocket;
use rocket_contrib::JsonValue;

pub fn rocket() -> (Rocket, Option<common::db::RequestContext>) {
    let pool = util::db::establish_connection_pool();
    let conn = Some(common::db::RequestContext(util::Context::new(
        pool.get().expect("database connection for testing"),
    )));

    let rocket = rocket::ignite()
        .manage(pool)
        .catch(catchers![not_found])
        .mount("/api/user", core::user::routes());

    (rocket, conn)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
