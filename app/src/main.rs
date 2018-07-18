#![feature(rust_2018_preview)]

extern crate env_logger;
extern crate failure;
extern crate fina_app_lib;
extern crate log;

fn main() {
    ::std::env::set_var("RUST_LOG", "info,cargo=WARN,fina=DEBUG");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    ::std::env::set_var(
        "DATABASE_URL",
        "postgres://billac:billac@localhost/billacdb",
    );
    env_logger::init();

    fina_app_lib::common::run();
}
