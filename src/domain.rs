use error;
use extern::reqwest;
use path::Path;
use request_information;
use std::cell;
use std::fmt;
use std::rc;

#[derive(Debug, Clone)]
pub struct Domain {
    info: rc::Rc<cell::RefCell<request_information::RequestInformation>>,
}

impl Domain {
    pub fn new(domain: &str) -> Self {
        let domain = domain.to_string();
        let info = request_information::RequestInformation::new(domain);

        Domain {
            info: rc::Rc::new(cell::RefCell::new(info)),
        }
    }

    /// Pushes the key/value combination onto the path as a query parameter.
    pub fn query_param(
        &mut self,
        key: &str,
        value: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        self.info.borrow_mut().add_query_param(key, value)?;

        Ok(())
    }

    pub fn header(&mut self, key: &'static str, value: &impl fmt::Display) {
        self.info.borrow_mut().add_header(key, value);
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

    fn new_path(&self, method: reqwest::Method) -> Path {
        Path::new(method, rc::Rc::clone(&self.info))
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
        domain.query_param(&"type", &"donkeys");

        let path = domain.get().push(&"list");
        assert_eq!(
            path.to_string(),
            "https://api.example.com/list?type=donkeys"
        );
    }

    #[test]
    fn domain_with_base_query_and_path() {
        let mut domain = Domain::new("https://api.example.com/");
        domain.query_param(&"type", &"donkeys");

        let path = domain.get().push(&"list").query_param(&"length", &"long");
        assert_eq!(
            path.to_string(),
            "https://api.example.com/list?type=donkeys&length=long"
        );
    }
}
