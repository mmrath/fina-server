use crate::model::core::{User, UserSignup};
use crate::service::core::user;

use util;

use crate::common::db;
use util::error::Error;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    routes![self::get, self::sign_up,]
}

#[get("/<id>", format = "json")]
fn get(id: i64, conn: db::Conn) -> Result<Option<Json<User>>, Error> {
    user::find_by_id(&util::Context::new(conn.0), id).map(|o| o.map(|u| Json(u)))
}

#[post("/signup", format = "json", data = "<signup>")]
fn sign_up(signup: Json<UserSignup>, conn: db::Conn) -> Result<Json<User>, Error> {
    user::sign_up(&util::Context::new(conn.0), &signup.0).map(|u| Json(u))
}
