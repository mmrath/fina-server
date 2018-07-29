use fina_model::core::{User, UserLogin, UserSignUp};
use fina_service::core::user;
use rocket::Route;
use rocket_contrib::Json;

use crate::common::db::RequestContext;
use crate::common::ApiError;

pub fn routes() -> Vec<Route> {
    routes![self::get, self::sign_up, self::login, self::activate]
}

#[get("/<id>", format = "json")]
fn get(id: i64, context: RequestContext) -> Result<Json<User>, ApiError> {
    user::find_by_id(&context, id)
        .map(|u| Json(u))
        .map_err(|err| err.into())
}

#[post("/signup", format = "json", data = "<signup>")]
fn sign_up(signup: Json<UserSignUp>, context: RequestContext) -> Result<Json<User>, ApiError> {
    user::sign_up(&context, &signup.0)
        .map(|u| Json(u))
        .map_err(|err| err.into())
}

#[post("/login", format = "json", data = "<login>")]
fn login(login: Json<UserLogin>, context: RequestContext) -> Result<Json<User>, ApiError> {
    user::login(&context, &login.username, &login.password)
        .map(|u| Json(u))
        .map_err(|err| err.into())
}

#[get("/activate/<token>")]
fn activate(token: String, context: RequestContext) -> Result<(), ApiError> {
    user::activate(&context, &token).map_err(|err| err.into())
}
