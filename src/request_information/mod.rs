use crate::error;
use serde;
use std::fmt;
use std::fmt::Write;

pub(crate) mod headers;
pub(crate) mod query_builder;

/// This is url bits + headers + query parameters.
/// The stuff that helps to make up a request.
///
/// It's abstracted away from Domain and Path, because they both need
/// both. A Path also needs access to the Domain's information as a
/// basis, and I don't want to have to clone all the information.
#[derive(Debug, Clone)]
pub(crate) struct RequestInformation {
    url: String,
    query: query_builder::QueryBuilder,
    headers: headers::Headers,
}

impl RequestInformation {
    pub(crate) fn new(url: String) -> Self {
        Self {
            url,
            query: query_builder::QueryBuilder::new(),
            headers: headers::Headers::new(),
        }
    }

    pub(crate) fn push_path_part(
        &mut self,
        part: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        write!(self.url, "/{}", part)?;

        Ok(())
    }

    pub(crate) fn push_path_part_partial(
        &mut self,
        part: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        write!(self.url, "{}", part)?;

        Ok(())
    }

    pub(crate) fn add_query_param(
        &mut self,
        key: &str,
        value: &impl fmt::Display,
    ) -> Result<(), error::Error> {
        self.query.add(key, value)
    }

    pub(crate) fn add_query_blob<B: serde::ser::Serialize + ?Sized>(
        &mut self,
        blob: &B,
    ) -> Result<(), error::Error> {
        self.query.add_blob(blob)
    }

    pub(crate) fn add_header(
        &mut self,
        key: &'static str,
        value: &impl fmt::Display,
    ) {
        self.headers.add(key, value);
    }

    pub(crate) fn for_each_header(
        &self,
        f: impl FnMut((&str, &str)),
    ) {
        self.headers.for_each(f)
    }
}

pub(crate) fn to_full_url(
    domain: &RequestInformation,
    parts: &RequestInformation,
) -> Result<String, fmt::Error> {
    let mut text = String::new();
    write!(
        &mut text,
        "{}",
        UrlFormatter {
            domain,
            parts
        }
    )?;
    Ok(text)
}

pub(crate) fn write_full_url(
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

struct UrlFormatter<'a> {
    domain: &'a RequestInformation,
    parts: &'a RequestInformation,
}

impl<'a> fmt::Display for UrlFormatter<'a> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}{}", self.domain.url.trim_right_matches(&"/"), self.parts.url)?;

        if !self.domain.query.is_empty() {
            write!(f, "?{}", self.domain.query)?;

            if !self.parts.query.is_empty() {
                write!(f, "&{}", self.parts.query)?;
            }
        } else if !self.parts.query.is_empty() {
            write!(f, "?{}", self.parts.query)?;
        }

        Ok(())
    }
}
