//! Error and Result module

/*
pub fn fail<T>(fail: T) -> impl Fn(failure::Error) -> Error<T> where T: Fail + Serialize + Copy{
   move |err| Error { context: err.context(fail) }
}
*/

pub trait Error {
    type Kind;

    fn kind(&self) -> Self::Kind;
    fn is_internal_err(&self) -> bool;
    fn to_internal_err<E>(err: E) -> Self
    where
        E: Into<::failure::Error>;
}
