mod string_stream;

use error;
use extern::futures::stream::Stream;
use extern::futures::Future;
use extern::hyper;
use extern::hyper_tls;
use extern::tokio;
use method;
use request_information;
use response;

/// This is a wrapper around Hyper. It has two aims.
///
///  * Bunch up common code in one place.
///  * Keep bridge code to Hyper (or whatever) isolated in one place.
#[derive(Debug, Clone)]
crate struct NativeClient {
    client: hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl NativeClient {
    crate fn new() -> Self {
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let client = hyper::client::Client::builder().build::<_, hyper::Body>(https);

        Self {
            client,
        }
    }

    crate fn request(
        &mut self,
        method: method::Method,
        domain_info: &request_information::RequestInformation,
        path_info: &request_information::RequestInformation,
        content: Option<String>,
    ) -> Result<impl Future<Item=response::Response, Error=error::Error>, error::Error> {
        let hyper_method = method_to_hyper(method);
        let url = request_information::to_full_url(domain_info, path_info)?;
        let body = content_to_body(content);

        let mut request_builder = hyper::Request::builder();
        request_builder.method(hyper_method).uri(&url);

        domain_info.for_each_header(|(key, value)| {
            request_builder.header(key, value);
        });
        path_info.for_each_header(|(key, value)| {
            request_builder.header(key, value);
        });

        let request = request_builder.body(body)?;
        let future = self
            .client
            .request(request)
            .map(|res| {
                let status = res.status().as_u16().into();
                let body = response_to_string(res);

                response::Response {
                    body,
                    status,
                }
            })
            .map_err(|err| error::Error::from(err));

        Ok(future)
    }

    crate fn request_blocking(
        &mut self,
        method: method::Method,
        domain_info: &request_information::RequestInformation,
        path_info: &request_information::RequestInformation,
        content: Option<String>,
    ) -> Result<response::Response, error::Error> {
        let future = self.request(method, domain_info, path_info, content)?;
        tokio::runtime::Runtime::new().unwrap().block_on(future)
    }
}

fn response_to_string(response: hyper::Response<hyper::body::Body>) -> String {
    response
        .into_body()
        .map_err(|_| ())
        .fold(vec![], |mut acc, chunk| {
            acc.extend_from_slice(&chunk);
            Ok(acc)
        })
        .and_then(|v| String::from_utf8(v).map_err(|_| ()))
        .wait()
        .unwrap()
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
        method::Method::Get => hyper::Method::GET,
        method::Method::Post => hyper::Method::POST,
        method::Method::Put => hyper::Method::PUT,
        method::Method::Delete => hyper::Method::DELETE,
        method::Method::Head => hyper::Method::HEAD,
        method::Method::Options => hyper::Method::OPTIONS,
        method::Method::Connect => hyper::Method::CONNECT,
        method::Method::Patch => hyper::Method::PATCH,
        method::Method::Trace => hyper::Method::TRACE,
    }
}