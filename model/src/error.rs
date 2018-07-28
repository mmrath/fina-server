use util::error::Error;
use diesel::result::Error as DieselError;


#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum DataErrorKind {
    #[fail(display = "Internal DB error")]
    Internal,

    #[fail(display = "No data found")]
    NotFound,

    #[fail(
        display = "Incorrect result size: expected:{} found:{}",
        _0,
        _1
    )]
    IncorrectResultSize(usize, usize),

    #[fail(display = "Unknown error")]
    __NonExhaustive,
}
error_kind!(DataError, DataErrorKind);


impl From<DieselError> for DataError {
    fn from(err: DieselError) -> Self {
        use failure;
        match err {
            DieselError::NotFound =>
                Self { inner: failure::Error::from(err).context(DataErrorKind::NotFound) },
            _ =>
                Self { inner: failure::Error::from(err).context(DataErrorKind::Internal) }
        }
    }
}