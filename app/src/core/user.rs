use actix::prelude::*;
use common::ServiceActor;
use model::core::{User, UserSignUp};
use service::core::user::{find_by_id, sign_up, SignUpError, SignUpErrorKind, DataError, DataErrorKind};
use util;
use failure::{ResultExt, Fail};
use actix_web::{App, AsyncResponder, FutureResponse, HttpResponse, Json, Path, State};
use common::AppState;
use futures::Future;
use http::StatusCode;

pub(crate) fn config(app: App<AppState>) -> App<AppState> {
    app.resource("/user/signup", |r| r.post().with(register_user))
        .resource("/user/{id}", |r| r.get().with(find_user))
}

/// Async request handler
pub(crate) fn find_user((id, state): (Path<i64>, State<AppState>)) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    info!("Finding user with id {}", id.clone());
    state
        .service_actor
        .send(FindUserMsg(id.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(err) => {
                let resp =
                    HttpResponse::with_body(StatusCode::BAD_REQUEST, format!("{:?}", err.cause()));
                Ok(resp)
            }
        })
        .responder()
}

pub(crate) fn register_user(
    (user_reg, state): (Json<UserSignUp>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .service_actor
        .send(RegisterUserMsg(user_reg.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(err) => {
                let resp =
                    HttpResponse::with_body(StatusCode::BAD_REQUEST, format!("{:?}", err.cause()));
                debug!("Error signing up user {:?}", err);
                Ok(resp)
            }
        })
        .responder()
}



















pub struct RegisterUserMsg(pub UserSignUp);

impl Message for RegisterUserMsg {
    type Result = Result<User, SignUpError>;
}

impl Handler<RegisterUserMsg> for ServiceActor {
    type Result = Result<User, SignUpError>;

    fn handle(&mut self, msg: RegisterUserMsg, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get().context(SignUpErrorKind::Internal)?;
        info!("Register message is {:?}", msg.0);
        sign_up(&util::Context::new(conn), &msg.0)
    }
}

pub struct FindUserMsg(pub i64);

impl Message for FindUserMsg {
    type Result = Result<User, DataError>;
}

impl Handler<FindUserMsg> for ServiceActor {
    type Result = Result<User, DataError>;

    fn handle(&mut self, msg: FindUserMsg, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get().context(DataErrorKind::Internal)?;
        find_by_id(&util::Context::new(conn), msg.0)
    }
}


