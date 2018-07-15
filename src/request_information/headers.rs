use std::fmt;

#[derive(Debug, Clone)]
crate struct Headers {
    headers: Option<Vec<(String, String)>>,
}

impl Headers {
    crate fn new() -> Self {
        Self {
            headers: None,
        }
    }

    /// Stores the header.
    crate fn add(
        &mut self,
        key: &str,
        value: &impl fmt::Display,
    ) {
        if let None = self.headers {
            self.headers = Some(Vec::new());
        }

        self.headers.as_mut().unwrap().push((key.to_string(), value.to_string()));
    }

    /// An iterator over all header key => value pairs.
    crate fn for_each(
        &self,
        mut f: impl FnMut((&str, &str)),
    ) {
        if let Some(ref headers) = self.headers {
            headers.iter().for_each(|(ref key, ref value)| f((key, value)))
        }
    }
}
