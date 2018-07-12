use std::fmt;
use std::slice;

#[derive(Debug, Clone)]
crate struct Headers {
    headers: Option<Vec<(&'static str, String)>>,
}

impl Headers {
    crate fn new() -> Self {
        Self { headers: None }
    }

    /// Stores the header.
    crate fn add(&mut self, key: &'static str, value: &impl fmt::Display) {
        if let None = self.headers {
            self.headers = Some(Vec::new());
        }

        self.headers
            .as_mut()
            .unwrap()
            .push((key, value.to_string()));
    }

    /// An iterator over all header key => value pairs.
    crate fn iter(&self) -> slice::Iter<(&'static str, String)> {
        match self.headers {
            Some(headers) => headers.iter(),
            None => Vec::new().iter(),
        }
    }
}
