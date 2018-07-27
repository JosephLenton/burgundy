use error;
use method;
use native_client;
use path::Path;
use request_information;
use std::cell;
use std::fmt;
use std::rc;

#[derive(Debug)]
pub struct Domain {
    client: rc::Rc<cell::RefCell<native_client::NativeClient>>,
    info: rc::Rc<cell::RefCell<request_information::RequestInformation>>,
}

impl Domain {
    pub fn new(domain: &str) -> Self {
        let domain = domain.to_string();
        let client = rc::Rc::new(cell::RefCell::new(native_client::NativeClient::new()));
        let info =
            rc::Rc::new(cell::RefCell::new(request_information::RequestInformation::new(domain)));

        Domain {
            client,
            info,
        }
    }

    /// Pushes the key/value combination onto the path as a query parameter.
    pub fn query(
        &mut self,
        key: &str,
        value: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        self.info.borrow_mut().add_query_param(key, value)?;

        Ok(())
    }

    pub fn header(
        &mut self,
        key: &'static str,
        value: &impl fmt::Display,
    ) {
        self.info.borrow_mut().add_header(key, value);
    }

    pub fn get(&self) -> Path {
        self.new_path(method::Method::Get)
    }

    pub fn post(&self) -> Path {
        self.new_path(method::Method::Post)
    }

    pub fn put(&self) -> Path {
        self.new_path(method::Method::Put)
    }

    pub fn delete(&self) -> Path {
        self.new_path(method::Method::Delete)
    }

    pub fn patch(&self) -> Path {
        self.new_path(method::Method::Patch)
    }

    fn new_path(
        &self,
        method: method::Method,
    ) -> Path {
        Path::new(method, rc::Rc::clone(&self.client), rc::Rc::clone(&self.info))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn domain_no_end_slash() {
        let domain = Domain::new("https://api.example.com").get();
        assert_eq!(domain.to_string(), "https://api.example.com");
    }

    #[test]
    fn domain_should_strip_slash() {
        let domain = Domain::new("https://api.example.com/").get();
        assert_eq!(domain.to_string(), "https://api.example.com");
    }

    #[test]
    fn domain_with_base_query() {
        let mut domain = Domain::new("https://api.example.com/");
        domain.query(&"type", &"donkeys");

        let path = domain.get().push(&"list");
        assert_eq!(path.to_string(), "https://api.example.com/list?type=donkeys");
    }

    #[test]
    fn domain_with_base_query_and_path() {
        let mut domain = Domain::new("https://api.example.com/");
        domain.query(&"type", &"donkeys");

        let path = domain.get().push(&"list").query(&"length", &"long");
        assert_eq!(path.to_string(), "https://api.example.com/list?type=donkeys&length=long");
    }
}
