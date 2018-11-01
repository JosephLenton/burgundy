use crate::error;
use futures;
use hyper;

/// This is a fake future stream.
/// It wraps the `String` given, and when it is polled it just gets
/// returned.
///
/// Second, third, and other future polls, all return `Option::None`.
pub(crate) struct StringStream {
    content: Option<String>,
}

impl StringStream {
    /// Takes ownership of the `String`, turning it into a future stream.
    pub(crate) fn new(content: String) -> Self {
        Self {
            content: Some(content),
        }
    }
}

impl hyper::rt::Stream for StringStream {
    type Item = String;
    type Error = error::UnreachableError;

    /// This works the once to return the contents of the stream.
    /// The next time it's used it'll return `None`.
    fn poll(&mut self) -> Result<futures::Async<Option<Self::Item>>, Self::Error> {
        let content = if self.content.is_some() {
            self.content.take()
        } else {
            None
        };

        Ok(futures::Async::Ready(content))
    }
}
