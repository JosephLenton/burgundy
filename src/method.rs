/// Which type of request it being made.
#[derive(Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
    Trace,
}
