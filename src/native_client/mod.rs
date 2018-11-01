mod string_stream;

use crate::error;
use crate::method;
use crate::request_information;
use crate::response;
use futures::stream::Stream;
use futures::Future;
use hyper;
use hyper_tls;
use log::info;
use tokio;

/// This is a wrapper around Hyper. It has two aims.
///
///  * Bunch up common code in one place.
///  * Keep bridge code to Hyper (or whatever) isolated in one place.
#[derive(Debug)]
pub(crate) struct NativeClient {
    client: hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    tokio_runtime: tokio::runtime::Runtime,
}

impl NativeClient {
    pub(crate) fn new() -> Self {
        info!("new native client");
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        let tokio_executor = tokio_runtime.executor();
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let client = hyper::client::Client::builder()
            .executor(tokio_executor)
            .build::<_, hyper::Body>(https);

        info!("done making new native client");

        Self {
            client,
            tokio_runtime,
        }
    }

    pub(crate) fn request_blocking(
        &mut self,
        method: method::Method,
        domain_info: &request_information::RequestInformation,
        path_info: &request_information::RequestInformation,
        content: Option<String>,
    ) -> Result<response::Response, error::Error> {
        info!("making blocking request");
        let future = self.request(method, domain_info, path_info, content)?;

        info!("call blocking request");
        let response = self.tokio_runtime.block_on(future);

        info!("done making blocking request");
        response
    }

    pub(crate) fn request(
        &mut self,
        method: method::Method,
        domain_info: &request_information::RequestInformation,
        path_info: &request_information::RequestInformation,
        body_str: Option<String>,
    ) -> Result<impl Future<Item = response::Response, Error = error::Error>, error::Error> {
        info!("make request");
        let hyper_method = method_to_hyper(method);
        let url = request_information::to_full_url(domain_info, path_info)?;
        let mut request_builder = hyper::Request::builder();

        info!("making request to {}", url);
        request_builder.method(hyper_method).uri(&url);

        info!("set headers");
        domain_info.for_each_header(|(key, value)| {
            info!("set domain header '{}' to '{}'", key, value);
            request_builder.header(key, value);
        });
        path_info.for_each_header(|(key, value)| {
            info!("set path header '{}' to '{}'", key, value);
            request_builder.header(key, value);
        });

        info!("turn request into body");
        let body = content_to_body(body_str);
        let request = request_builder.body(body)?;

        info!("make request future");
        let future = self
            .client
            .request(request)
            .map(|res| {
                info!("transform request to response object");
                let status = res.status().as_u16().into();
                let body = response_to_string(res);
                info!("transform request to response object, has status {}", status);

                response::Response {
                    body,
                    status,
                }
            })
            .map_err(|err| error::Error::from(err));

        info!("done making request");
        Ok(future)
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

pub(crate) fn content_to_body(maybe_content: Option<String>) -> hyper::Body {
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
