use extern::http;
use extern::hyper;
use extern::serde_json;
use response;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, self::Error>;

/// Represents the errors possible to fall out from Burgundy.
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Error whilst formatting")]
    FormatError {
        /// The underlying error.
        #[cause]
        error: fmt::Error,
    },

    #[fail(display = "Failed to deserialize")]
    DeserializationError {
        /// The underlying error.
        #[cause]
        error: serde_json::error::Error,

        /// The text that was send to Serde. Useful for debugging.
        text: String,
    },

    #[fail(display = "Http error {}", error)]
    HttpError {
        /// The underlying error.
        #[cause]
        error: http::Error,
    },

    #[fail(display = "Network error {}", error)]
    NetworkError {
        /// The underlying error.
        #[cause]
        error: hyper::Error,
    },

    /// For HTTP requests which do not return 200.
    #[fail(display = "Http request was not ok, status {}", status)]
    RequestNotOk {
        status: u32,
        body: String,
    },
}

impl Error {
    /// Creates a new deserialization error.
    crate fn new_deserialization_error(
        error: serde_json::error::Error,
        text: String,
    ) -> Self {
        Error::DeserializationError {
            error: error,
            text: text,
        }
    }

    crate fn new_request_not_ok(response: response::Response) -> Self {
        Error::RequestNotOk {
            status: response.status,
            body: response.body,
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FormatError {
            error: err,
        }
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        Error::HttpError {
            error: err,
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::NetworkError {
            error: err,
        }
    }
}

/// Sometimes we have to add an error, even though we know it will never
/// happen.
///
/// This is for those times.
#[derive(Debug)]
pub(crate) struct UnreachableError;

impl error::Error for UnreachableError {
}

impl fmt::Display for UnreachableError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "This error should never be reached. If you can see it, then please report it as a bug.")
    }
}
