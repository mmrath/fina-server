use crate::model::core::User;
use crate::service::core::user;
use util;

use crate::common::db;
use failure::Error;

use rocket_contrib::Json;

#[get("/<id>", format = "json")]
fn get(id: i64, conn: db::Conn) -> Result<Option<Json<User>>, Error> {
    user::find_by_id(&util::Context::new(conn.0), id).map(|o| o.map(|u| Json(u)))
}
