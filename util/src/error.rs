//! Error and Result module
use erased_serde;
use failure::{self, Backtrace, Context, Fail};
pub use failure::ResultExt;
use http::StatusCode;
use serde::{Serialize, Serializer};
use std::{fmt::{self, Debug, Display}, result};
use std::sync::Mutex;

/// A specialized [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
/// for actix web operations
///
/// This typedef is generally used to avoid writing out
/// `actix_web::error::Error` directly and is otherwise a direct mapping to
/// `Result`.
///pub type Result<T, E = Error> = result::Result<T, E>;

/// General purpose actix web error.
///
/// An actix web error is used to carry errors from `failure` or `std::error`
/// through actix in a convenient way.  It can be created through through
/// converting errors with `into()`.
///
/// Whenever it is created from an external object a response error is created
/// for it that can be used to create an http response from it this means that
/// if you have access to an actix `Error` you can always get a
/// `ResponseError` reference from it.
#[derive(Debug)]
pub struct Error<T: Fail + Serialize> {
    context: Context<T>
}

impl<T: Fail + Serialize> Fail for Error<T> {
    fn cause(&self) -> Option<&Fail> {
        self.context.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.context.backtrace()
    }
}


impl<T: Fail + Serialize> Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.context, f)
    }
}


impl<T: Fail + Serialize> Error<T> {
    pub fn kind(&self) -> &T {
        self.context.get_context()
    }
}

impl<T: Fail + Serialize> From<T> for Error<T> {
    fn from(kind: T) -> Error<T> {
        Error { context: Context::new(kind) }
    }
}

impl<T: Fail + Serialize> From<Context<T>> for Error<T> {
    fn from(context: Context<T>) -> Error<T> {
        Error { context }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum DataError {
    #[fail(display = "DB error")]
    Unknown,

    #[fail(display = "Incorrect result size: expected:{} found:{}", _0, _1)]
    IncorrectResultSize(usize, usize),

}




pub fn fail<T>(fail: T) -> impl Fn(failure::Error) -> Error<T> where T: Fail + Serialize + Copy{
   move |err| Error { context: err.context(fail) }
}


macro_rules! error_kind {
    ( $error: ident, $error_kind: ident ) => {

    }
}