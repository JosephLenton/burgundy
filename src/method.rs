/// Which type of request it being made.
#[derive(Debug, Copy, Clone)]
crate enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
}
