use extern::failure::Error;
use extern::reqwest;
use extern::serde;
use request_information;
use std::cell;
use std::fmt;
use std::rc;

/// To make a `Path`, you need to use the `Domain` first.
/// From that you can generate `Path` objects.
#[derive(Debug, Clone)]
pub struct Path {
    method: reqwest::Method,
    domain_info: rc::Rc<cell::RefCell<request_information::RequestInformation>>,
    info: request_information::RequestInformation,
}

impl Path {
    crate fn new(
        method: reqwest::Method,
        domain_info: rc::Rc<cell::RefCell<request_information::RequestInformation>>,
    ) -> Self {
        let info = request_information::RequestInformation::new(String::new());

        Self {
            method,
            domain_info,
            info,
        }
    }

    pub fn push(mut self, next: &impl fmt::Display) -> Self {
        self.info.push_path_part(next);

        self
    }

    pub fn push_query(mut self, key: &str, value: &impl fmt::Display) -> Self {
        self.info.push_query(key, value);

        self
    }

    pub fn execute<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        Ok(self.request()?.json::<T>()?)
    }

    pub fn execute_raw(self) -> Result<String, Error> {
        Ok(self.request()?.text()?)
    }

    fn request(self) -> Result<reqwest::Response, Error> {
        let url = self.to_string();
        let client = reqwest::Client::new();

        let mut request_builder = client.request(self.method, &url);
        let response = request_builder.send()?;

        Ok(response)
    }
}

impl<'a> fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        request_information::write_full_url(f, &self.domain_info.borrow(), &self.info)
    }
}

#[cfg(test)]
mod test {
    use super::super::Domain;

    #[test]
    fn push_works() {
        let domain = Domain::new(&"https://api.example.com");
        let path = domain
            .get()
            .push(&"org")
            .push(&"Microsoft")
            .push(&"projects");

        assert_eq!(
            path.to_string(),
            "https://api.example.com/org/Microsoft/projects"
        );
    }

    #[test]
    fn domain_should_strip_slash() {
        let domain = Domain::new(&"https://api.example.com");
        let path = domain.get().push(&"list").push(&123);

        assert_eq!(path.to_string(), "https://api.example.com/list/123");
    }

    #[test]
    fn query_parameters() {
        let domain = Domain::new("https://api.example.com");
        let path = domain
            .get()
            .push(&"list")
            .push_query(&"size", &50)
            .push_query(&"index", &2);

        assert_eq!(
            path.to_string(),
            "https://api.example.com/list?size=50&index=2"
        );
    }
}
