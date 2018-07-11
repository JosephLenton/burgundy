use extern::reqwest;
use std::fmt;

#[derive(Debug, Clone)]
crate struct Headers {
    headers: Option<Vec<(&'static str, String)>>,
}

impl Headers {
    crate fn new() -> Self {
        Self { headers: None }
    }

    crate fn add(&mut self, key: &'static str, value: &impl fmt::Display) {
        if let None = self.headers {
            self.headers = Some(Vec::new());
        }

        self.headers
            .as_mut()
            .unwrap()
            .push((key, value.to_string()));
    }

    crate fn copy_headers(&self, dest_headers: &mut reqwest::header::Headers) {
        if let Some(ref headers) = self.headers {
            headers.iter().for_each(|(k, v)| {
                dest_headers.set_raw(*k, v.as_str());
            });
        }
    }
}
