mod string_stream;

use error;
use extern::futures::stream::Stream;
use extern::hyper;
use extern::hyper::rt::Future;
use method;
use request_information;
use response;

/// This is a wrapper around Hyper. It has two aims.
///
///  * Bunch up common code in one place.
///  * Keep bridge code to Hyper (or whatever) isolated in one place.
#[derive(Debug, Clone)]
crate struct NativeClient {
    client: hyper::client::Client<hyper::client::HttpConnector>,
}

impl NativeClient {
    crate fn new() -> Self {
        let client = hyper::client::Client::new();

        Self { client }
    }

    crate fn request(
        &mut self,
        method: method::Method,
        domain_info: &request_information::RequestInformation,
        path_info: &request_information::RequestInformation,
        content: Option<String>,
    ) -> Result<response::Response, error::Error> {
        let hyper_method = method_to_hyper(method);
        let url = request_information::to_full_url(domain_info, path_info)?;
        let body = content_to_body(content);

        let request = hyper::Request::builder()
            .method(hyper_method)
            .uri(&url)
            .body(body)?;
        let response = self.client.request(request).wait()?;

        let status = response.status();
        let body = response_to_string(response);

        Ok(response::Response {
            body,
            status: status.as_u16().into(),
        })
    }
}

fn response_to_string(response: hyper::Response<hyper::body::Body>) -> String {
    let mut body = String::new();

    response.into_body().for_each(|chunk| {
        // I don't like that we go chunk to String to write.
        // However I don't know how to get rid of it.
        let chunk_str = String::from_utf8_lossy(&chunk);
        body.push_str(&chunk_str);
    });

    body
}

crate fn content_to_body(maybe_content: Option<String>) -> hyper::Body {
    if let Some(content) = maybe_content {
        let stream = string_stream::StringStream::new(content);
        hyper::Body::wrap_stream(stream)
    } else {
        hyper::Body::empty()
    }
}

fn method_to_hyper(method: method::Method) -> hyper::Method {
    match method {
        Get => hyper::Method::GET,
        Post => hyper::Method::POST,
        Put => hyper::Method::PUT,
        Delete => hyper::Method::DELETE,
        Head => hyper::Method::HEAD,
        Options => hyper::Method::OPTIONS,
        Connect => hyper::Method::CONNECT,
        Post => hyper::Method::POST,
    }
}
