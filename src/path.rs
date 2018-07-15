use error;
use extern::serde;
use extern::serde_json;
use method;
use native_client;
use request_information;
use response;
use std::cell;
use std::fmt;
use std::rc;

/// To make a `Path`, you need to use the `Domain` first.
/// From that you can generate `Path` objects.
#[derive(Debug, Clone)]
pub struct Path {
    method: method::Method,
    client: rc::Rc<cell::RefCell<native_client::NativeClient>>,
    domain_info: rc::Rc<cell::RefCell<request_information::RequestInformation>>,
    info: request_information::RequestInformation,
}

impl Path {
    crate fn new(
        method: method::Method,
        client: rc::Rc<cell::RefCell<native_client::NativeClient>>,
        domain_info: rc::Rc<cell::RefCell<request_information::RequestInformation>>,
    ) -> Self {
        let info = request_information::RequestInformation::new(String::new());

        Self {
            method,
            client,
            domain_info,
            info,
        }
    }

    pub fn push(
        mut self,
        next: &impl fmt::Display,
    ) -> Self {
        self.info.push_path_part(next);

        self
    }

    pub fn query(
        mut self,
        key: &str,
        value: &impl fmt::Display,
    ) -> Self {
        self.info.add_query_param(key, value);

        self
    }

    pub fn header(
        &mut self,
        key: &'static str,
        value: &impl fmt::Display,
    ) {
        self.info.add_header(key, value);
    }

    /// Executes the path, and deserializes what comes back.
    pub fn execute_as_json<T: serde::de::DeserializeOwned>(self) -> Result<T, error::Error> {
        deserialize::<T>(self.execute_as_string()?)
    }

    /// Sends the request, returns the response as just a String.
    pub fn execute_as_string(self) -> Result<String, error::Error> {
        string_or_error(self.execute(None))
    }

    fn execute(
        self,
        body: Option<String>,
    ) -> Result<response::Response, error::Error> {
        self.client.borrow_mut().request(self.method, &self.domain_info.borrow(), &self.info, body)
    }
}

impl<'a> fmt::Display for Path {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        request_information::write_full_url(f, &self.domain_info.borrow(), &self.info)
    }
}

fn string_or_error(
    response: Result<response::Response, error::Error>,
) -> Result<String, error::Error> {
    response.and_then(|r| {
        if r.status == 200 {
            Ok(r.body)
        } else {
            Err(error::Error::new_request_not_ok(r))
        }
    })
}

fn deserialize<T: serde::de::DeserializeOwned>(body: String) -> Result<T, error::Error> {
    Ok(serde_json::from_str::<T>(&body)
        .map_err(|err| error::Error::new_deserialization_error(err, body))?)
}

#[cfg(test)]
mod test {
    use super::super::Domain;

    #[test]
    fn push_works() {
        let domain = Domain::new(&"https://api.example.com");
        let path = domain.get().push(&"org").push(&"Microsoft").push(&"projects");

        assert_eq!(path.to_string(), "https://api.example.com/org/Microsoft/projects");
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
        let path = domain.get().push(&"list").query_param(&"size", &50).query_param(&"index", &2);

        assert_eq!(path.to_string(), "https://api.example.com/list?size=50&index=2");
    }
}
