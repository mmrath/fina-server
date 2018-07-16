pub mod user;

use actix_web::App;
use super::common::AppState;

pub(crate) fn config(app: App<AppState>) -> App<AppState> {
    user::config(app)
}