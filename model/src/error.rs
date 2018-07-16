#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum DbErrorKind {
    #[fail(display = "Internal DB error")]
    Internal,

    #[fail(display = "Incorrect result size: expected:{} found:{}", _0, _1)]
    IncorrectResultSize(usize, usize),

    #[fail(display = "Unknown error")]
    __NonExhaustive

}
error_kind!(DbError, DbErrorKind);