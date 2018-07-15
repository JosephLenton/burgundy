use error;
use std::fmt;
use std::fmt::Write;

const QUERY_STRING_START_SIZE: usize = 20;

#[derive(Debug, Clone)]
crate struct QueryBuilder {
    contents: Option<String>,
}

impl QueryBuilder {
    /// Trivial constructor.
    crate fn new() -> Self {
        QueryBuilder {
            contents: None,
        }
    }

    /// True if this is empty.
    crate fn is_empty(&self) -> bool {
        if let Some(ref query) = self.contents {
            query.len() == 0
        } else {
            true
        }
    }

    /// Pushes the key/value combination onto the path as a query parameter.
    crate fn add(
        &mut self,
        key: &str,
        value: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        if let None = self.contents {
            let query = String::with_capacity(QUERY_STRING_START_SIZE);
            self.contents = Some(query);
        }

        let query_str = self.contents.as_mut().unwrap();

        if query_str.len() > 0 {
            write!(query_str, "&")?;
        }

        write!(query_str, "{}={}", &key, &value)?;

        Ok(())
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
}
