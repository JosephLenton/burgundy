use extern::http;
use extern::hyper;
use extern::serde_json;
use extern::serde_urlencoded;
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

    #[fail(display = "Failed to deserialize response")]
    DeserializationError {
        /// The underlying error.
        #[cause]
        error: serde_json::Error,

        /// The text that was send to Serde. Useful for debugging.
        text: String,
    },

    #[fail(display = "Error serializing the blob into a query {}", error)]
    SerializeQueryError {
        /// The underlying error.
        #[cause]
        error: serde_urlencoded::ser::Error,
    },

    #[fail(display = "Failed to serialize the body for sending {}", error)]
    SerializeBodyError {
        /// The underlying error.
        #[cause]
        error: serde_json::Error,
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
        error: serde_json::Error,
        text: String,
    ) -> Self {
        Error::DeserializationError {
            error,
            text,
        }
    }

    crate fn new_request_not_ok(response: response::Response) -> Self {
        Error::RequestNotOk {
            status: response.status,
            body: response.body,
        }
    }

    crate fn new_serialize_query_error(error: serde_urlencoded::ser::Error) -> Self {
        Error::SerializeQueryError {
            error,
        }
    }

    crate fn new_serialize_body_error(error: serde_json::Error) -> Self {
        Error::SerializeBodyError {
            error,
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        Error::FormatError {
            error,
        }
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Error::HttpError {
            error,
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::NetworkError {
            error,
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
