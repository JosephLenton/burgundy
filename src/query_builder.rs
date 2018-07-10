
use std::fmt;

const QUERY_STRING_START_SIZE : usize = 20;

#[derive(Debug, Clone)]
crate struct QueryBuilder {
  contents : Option<String>
}

impl QueryBuilder {
  /// Trivial constructor.
  crate fn new() -> Self {
    QueryBuilder {
      contents : None
    }
  }

  /// Pushes the key/value combination onto the path as a query parameter.
  crate fn push_query<S: fmt::Display>(&mut self, key:&str, value:S) {
    if let None = self.contents {
      let query = String::with_capacity( QUERY_STRING_START_SIZE );
      self.contents = Some(query);
    }

    let query_str = self.contents.as_mut().unwrap();

    if query_str.len() == 0 {
      query_str.push_str(&"?");
    } else {
      query_str.push_str(&"&");
    }

    query_str.push_str(&key);
    query_str.push_str(&"=");
    query_str.push_str(&value.to_string());
  }
}

impl fmt::Display for QueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    assert_eq!(query.to_string(), "" );
  }

  #[test]
  fn one_query_key_value() {
    let mut query = QueryBuilder::new();
    query.push_query( "key", "value" );

    assert_eq!(query.to_string(), "?key=value" );
  }

  #[test]
  fn multiple_query_key_value() {
    let mut query = QueryBuilder::new();
    query.push_query( "key", "value" );
    query.push_query( "donkeyfy", true );
    query.push_query( "num_cats", 123 );

    assert_eq!(query.to_string(), "?key=value&donkeyfy=true&num_cats=123" );
  }
}