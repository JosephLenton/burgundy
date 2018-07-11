use std::fmt;

#[derive(Debug, Clone)]
crate struct Headers {
    headers: Option<Vec<(String, String)>>,
}

impl Headers {
    crate fn new() -> Self {
        Self { headers: None }
    }

    crate fn add(&mut self, key: &str, value: &impl fmt::Display) {
        if let None = self.headers {
            self.headers = Some(Vec::new());
        }

        self.headers
            .as_mut()
            .unwrap()
            .push((key.to_string(), value.to_string()));
    }
}
