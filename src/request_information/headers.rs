use std::fmt;
use std::slice;

#[derive(Debug, Clone)]
crate struct Headers {
    headers: Option<Vec<(String, String)>>,
}

impl Headers {
    crate fn new() -> Self {
        Self { headers: None }
    }

    /// Stores the header.
    crate fn add(&mut self, key: &str, value: &impl fmt::Display) {
        if let None = self.headers {
            self.headers = Some(Vec::new());
        }

        self.headers
            .as_mut()
            .unwrap()
            .push((key.to_string(), value.to_string()));
    }

    /// An iterator over all header key => value pairs.
    crate fn for_each<F, T>(&self, f: F)
    where
        F: FnMut(()),
    {
        if Some(headers) = self.headers {
            headers.iter().for_each(f)
        }
    }
}
