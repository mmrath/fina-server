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
extern crate uuid;

pub use self::context::Context;
pub use self::db::*;
pub use self::util_functions::*;
use failure::Error;

mod macros;

mod context;
mod db;
mod util_functions;

pub fn default_context() -> Result<Context, Error> {
    let pc = DB_POOL
        .clone()
        .get()
        .map_err(|_| format_err!("Unable to retrieve connection from pool"))?;
    Ok(Context::new(pc))
}
