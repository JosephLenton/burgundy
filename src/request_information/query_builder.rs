use crate::error;
use serde;
use serde_urlencoded;
use std::fmt;
use std::fmt::Write;

const QUERY_STRING_START_SIZE: usize = 20;

#[derive(Debug, Clone)]
pub(crate) struct QueryBuilder {
    contents: Option<String>,
}

impl QueryBuilder {
    /// Trivial constructor.
    pub(crate) fn new() -> Self {
        QueryBuilder {
            contents: None,
        }
    }

    /// True if this is empty.
    pub(crate) fn is_empty(&self) -> bool {
        if let Some(ref query) = self.contents {
            query.len() == 0
        } else {
            true
        }
    }
    pub(crate) fn add_blob<B: serde::ser::Serialize + ?Sized>(
        &mut self,
        blob: &B,
    ) -> Result<(), error::Error> {
        let query_str = self.get_query_string_buffer();

        if query_str.len() > 0 {
            write!(query_str, "&")?;
        }

        let blob_str =
            serde_urlencoded::to_string(&blob).map_err(error::Error::new_serialize_query_error)?;
        write!(query_str, "{}", &blob_str)?;

        Ok(())
    }

    /// Pushes the key/value combination onto the path as a query parameter.
    pub(crate) fn add(
        &mut self,
        key: &str,
        value: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        let query_str = self.get_query_string_buffer();

        if query_str.len() > 0 {
            write!(query_str, "&")?;
        }

        write!(query_str, "{}={}", &key, &value)?;

        Ok(())
    }

    fn get_query_string_buffer(&mut self) -> &mut String {
        if let None = self.contents {
            let query = String::with_capacity(QUERY_STRING_START_SIZE);
            self.contents = Some(query);
        }

        self.contents.as_mut().unwrap()
    }
}

impl fmt::Display for QueryBuilder {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        if let Some(ref contents) = self.contents {
            write!(f, "{}", contents)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_query_is_blank() {
        let query = QueryBuilder::new();

        assert_eq!(query.to_string(), "");
    }

    #[test]
    fn one_query_key_value() {
        let mut query = QueryBuilder::new();
        query.add(&"key", &"value");

        assert_eq!(query.to_string(), "key=value");
    }

    #[test]
    fn multiple_query_key_value() {
        let mut query = QueryBuilder::new();
        query.add(&"key", &"value");
        query.add(&"donkeyfy", &true);
        query.add(&"num_cats", &123);

        assert_eq!(query.to_string(), "key=value&donkeyfy=true&num_cats=123");
    }

    #[test]
    fn query_with_blob() {
        #[derive(Serialize)]
        struct Blob {
            pages: u32,
            name: &'static str,
        }

        let mut query = QueryBuilder::new();
        let blob = Blob {
            pages: 123,
            name: "abc_999_xyz",
        };

        query.add_blob(&blob);

        assert_eq!(query.to_string(), "pages=123&name=abc_999_xyz");
    }

    #[test]
    fn query_with_blob_and_parts() {
        #[derive(Serialize)]
        struct Blob {
            pages: u32,
            name: &'static str,
        }

        let mut query = QueryBuilder::new();

        let blob = Blob {
            pages: 123,
            name: "abc_999_xyz",
        };

        query.add(&"donkeyfy", &true);
        query.add_blob(&blob);
        query.add(&"num_cats", &123);

        assert_eq!(query.to_string(), "donkeyfy=true&pages=123&name=abc_999_xyz&num_cats=123");
    }
}
