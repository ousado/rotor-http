use hyper::version::HttpVersion as Version;
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::uri::RequestUri;
use hyper::header::Headers;


#[derive(Debug)]
/// Request headers
///
/// We don't have a request object because it is structured differently
/// based on whether it is buffered or streaming, chunked or http2, etc.
///
/// Note: we do our base to keep Head object same for HTTP 1-2 and HTTPS
pub struct Head {
    pub version: Version,
    pub https: bool,
    pub method: Method,
    pub uri: RequestUri,
    pub headers: Headers,
}