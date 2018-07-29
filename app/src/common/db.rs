use fina_util::db::ConnectionPool;
use fina_util::Context;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

pub struct RequestContext(pub Context);

impl Deref for RequestContext {
    type Target = Context;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RequestContext {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RequestContext, ()> {
        let pool = request.guard::<State<ConnectionPool>>()?;
        pool.get()
            .map(|conn| Outcome::Success(RequestContext(Context::new(conn))))
            .unwrap_or_else(|_e| Outcome::Failure((Status::ServiceUnavailable, ())))
    }
}
