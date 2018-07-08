#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#![feature(rust_2018_preview)]
#![warn(rust_2018_compatibility)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate env_logger;
extern crate failure;
extern crate log;

extern crate fina_model as model;
extern crate fina_service as service;
extern crate fina_util as util;

mod common;
mod core;

use rocket::Rocket;
use rocket_contrib::JsonValue;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

pub fn rocket() -> (Rocket, Option<common::db::Conn>) {
    let pool = util::establish_connection_pool();
    let conn = if cfg!(test) {
        Some(common::db::Conn(
            pool.get().expect("database connection for testing"),
        ))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .catch(catchers![not_found])
        .mount("/api/hello", routes![hello])
        .mount("/api/user", routes![core::user::get]);

    (rocket, conn)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}