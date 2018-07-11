//!
//! All of the bridge code that works with Hyper is stored here.
//! It allows it to be contained.
//!
//! Why? Incase I want to rewrite this to use something else.
//!

use extern::hyper;
use method;
use request_information;

crate fn new_request(
    method: method::Method,
    domain_info: &request_information::RequestInformation,
    path_info: &request_information::RequestInformation,
) {
    let url = request_information::to_full_url(domain_info, path_info)?;
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
