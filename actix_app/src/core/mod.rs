pub mod user;

use super::common::AppState;
use actix_web::App;

pub(crate) fn config(app: App<AppState>) -> App<AppState> {
    user::config(app)
}
