
use std::fmt;
use extern::reqwest;
use path::Path;
use query_builder::QueryBuilder;

#[derive(Debug, Clone)]
pub struct Domain {
  domain: String,
  query: QueryBuilder,
  headers: reqwest::header::Headers,
}

impl Domain {
  pub fn new(domain:&str) -> Self {
    Domain {
      domain: domain.trim_right_matches(&"/").to_string(),
      query: QueryBuilder::new(),
      headers: reqwest::header::Headers::new(),
    }
  }

  /// Pushes the key/value combination onto the path as a query parameter.
  pub fn push_query<S: fmt::Display>(&mut self, key:&str, value:S) {
    self.query.push_query(key, value);
  }

  pub fn get(&self) -> Path {
    self.new_path(reqwest::Method::Get)
  }

  pub fn post(&self) -> Path {
    self.new_path(reqwest::Method::Post)
  }

  pub fn put(&self) -> Path {
    self.new_path(reqwest::Method::Put)
  }

  pub fn delete(&self) -> Path {
    self.new_path(reqwest::Method::Delete)
  }

  pub fn patch(&self) -> Path {
    self.new_path(reqwest::Method::Patch)
  }

  pub fn header(&mut self, key:&str, value:&str) {

  }

  fn new_path(&self, method:reqwest::Method) -> Path {
    Path::new(&self.domain, &self.query, method, &self.headers)
  }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.domain)
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn domain_no_end_slash() {
    let domain = Domain::new("https://api.example.com");
    assert_eq!(domain.to_string(), "https://api.example.com" );
  }

  #[test]
  fn domain_should_strip_slash() {
    let domain = Domain::new("https://api.example.com/");
    assert_eq!(domain.to_string(), "https://api.example.com" );
  }

  #[test]
  fn domain_with_base_query() {
    let mut domain = Domain::new("https://api.example.com/");
    domain.push_query("type", "donkeys");

    let path = domain.get().push("list");
    assert_eq!(path.to_string(), "https://api.example.com/list?type=donkeys" );
  }

  #[test]
  fn domain_with_base_query_and_path() {
    let mut domain = Domain::new("https://api.example.com/");
    domain.push_query("type", "donkeys");

    let path = domain.get().push("list").push_query("length", "long");
    assert_eq!(path.to_string(), "https://api.example.com/list?type=donkeys&length=long" );
  }
}
