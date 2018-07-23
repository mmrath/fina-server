use model::core::{User, UserSignUp};
use model::error::{DataError};
use service::core::user::{self, SignUpError};

use util;

use crate::common::db::{RequestContext};
use util::error::Error;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    routes![self::get, self::sign_up,]
}

#[get("/<id>", format = "json")]
fn get(id: i64, context: RequestContext) -> Result<Json<User>, DataError> {
    user::find_by_id(&context, id).map(|u| Json(u))
}

#[post("/signup", format = "json", data = "<signup>")]
fn sign_up(signup: Json<UserSignUp>, context: RequestContext) -> Result<Json<User>, SignUpError> {
    user::sign_up(&context, &signup.0).map(|u| Json(u))
}

/*
#[post("/activate", format = "json", data = "<signup>")]
fn sign_up(signup: Json<UserSignUp>, context: RequestContext) -> Result<Json<User>, SignUpError> {
    user::sign_up(&context, &signup.0).map(|u| Json(u))
}
*/

