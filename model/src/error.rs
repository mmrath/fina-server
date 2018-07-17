#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum DataErrorKind {
    #[fail(display = "Internal DB error")]
    Internal,

    #[fail(display = "No data found")]
    NotFound,

    #[fail(display = "Incorrect result size: expected:{} found:{}", _0, _1)]
    IncorrectResultSize(usize, usize),

    #[fail(display = "Unknown error")]
    __NonExhaustive

}
error_kind!(DataError, DataErrorKind);
