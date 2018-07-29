#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
#![feature(rust_2018_preview)]

extern crate env_logger;
extern crate failure;
extern crate fina_app_lib;
extern crate log;
extern crate rocket;
extern crate rocket_contrib;

extern crate fina_model as model;
extern crate fina_service as service;
extern crate fina_util as util;

fn main() {
    ::std::env::set_var("RUST_LOG", "info,cargo=WARN,fina=DEBUG");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    ::std::env::set_var(
        "DATABASE_URL",
        "postgres://billac:billac@localhost/billacdb",
    );
    env_logger::init();

    fina_app_lib::rocket().0.launch();
}
