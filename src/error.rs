use std::fmt;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Error whilst formatting")]
    FormatError {
        #[cause]
        error: fmt::Error,
    },
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FormatError { error: err }
    }
}
