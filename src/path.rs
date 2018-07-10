
use std::fmt;
use extern::reqwest;
use extern::serde;
use query_builder::QueryBuilder;
use extern::failure::Error;

/// To make a `Path`, you need to use the `Domain` first.
/// From that you can generate `Path` objects.
#[derive(Debug, Clone)]
pub struct Path {
  url: String,
  method: reqwest::Method,
  query: QueryBuilder,
  headers: reqwest::header::Headers,
}

impl Path {
  crate fn new(domain:&str, query:&QueryBuilder, method:reqwest::Method, headers:&reqwest::header::Headers) -> Self {
    Self {
      method,
      url: domain.to_string(),
      query: query.clone(),
      headers: headers.clone(),
    }
  }

  pub fn push<S: fmt::Display>(mut self, next:S) -> Self {
    self.url.push_str(&"/");
    self.url.push_str(&next.to_string());

    self
  }

  pub fn push_query<S: fmt::Display>(mut self, key:&str, value:S) -> Self {
    self.query.push_query(key, value);

    self
  }

  pub fn execute<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
    let url = self.to_string();
    let client = reqwest::Client::new();

    let mut request_builder = client.request(self.method, &url);
    let mut response = request_builder.send()?;

    Ok(response.json::<T>()?)
  }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}{}", self.url, self.query)
    }
}

#[cfg(test)]
mod test {
  use super::super::Domain;

  #[test]
  fn push_works() {
    let domain = Domain::new("https://api.example.com");
    let path = domain.get().push("org").push("Microsoft").push("projects");

    assert_eq!(path.to_string(), "https://api.example.com/org/Microsoft/projects" );
  }

  #[test]
  fn domain_should_strip_slash() {
    let domain = Domain::new("https://api.example.com");
    let path = domain.get().push("list").push(123);

    assert_eq!(path.to_string(), "https://api.example.com/list/123" );
  }

  #[test]
  fn query_parameters() {
    let domain = Domain::new("https://api.example.com");
    let path = domain.get().push("list").push_query("size", 50).push_query("index", 2);

    assert_eq!(path.to_string(), "https://api.example.com/list?size=50&index=2" );
  }
}
