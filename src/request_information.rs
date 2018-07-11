use std::fmt;
use std::fmt::Write;

crate mod headers;
crate mod query_builder;

/// This is url bits + headers + query parameters.
/// The stuff that helps to make up a request.
///
/// It's abstracted away from Domain and Path, because they both need
/// both. A Path also needs access to the Domain's information as a
/// basis, and I don't want to have to clone all the information.
#[derive(Debug, Clone)]
crate struct RequestInformation {
    url: String,
    query: query_builder::QueryBuilder,
    headers: headers::Headers,
}

impl RequestInformation {
    crate fn new(url: String) -> Self {
        Self {
            url,
            query: query_builder::QueryBuilder::new(),
            headers: headers::Headers::new(),
        }
    }

    crate fn push_path_part(&mut self, part: &impl fmt::Display) {
        write!(self.url, "/{}", part);
    }

    crate fn add_query_param(&mut self, key: &str, value: &impl fmt::Display) {
        self.query.add(key, value);
    }

    crate fn add_header(&mut self, key: &str, value: &impl fmt::Display) {
        self.headers.add(key, value);
    }
}

crate fn write_full_url(
    f: &mut fmt::Formatter,
    domain: &RequestInformation,
    parts: &RequestInformation,
) -> fmt::Result {
    write!(f, "{}{}", domain.url.trim_right_matches(&"/"), parts.url)?;

    if !domain.query.is_empty() {
        write!(f, "?{}", domain.query)?;

        if !parts.query.is_empty() {
            write!(f, "&{}", parts.query)?;
        }
    } else if !parts.query.is_empty() {
        write!(f, "?{}", parts.query)?;
    }

    Ok(())
}
