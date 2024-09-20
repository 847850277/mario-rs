use http::{Extensions, HeaderMap, HeaderValue, Method, Uri, Version};
use hyper::body::Incoming;


#[derive(Debug)]
pub struct Request {
    pub head: Head,
    pub body: Incoming,
}

#[derive(Debug)]
pub struct Head {
    /// The request's method
    pub method: Method,

    /// The request's URI
    pub uri: Uri,

    /// The request's version
    pub version: Version,

    /// The request's headers
    pub headers: HeaderMap<HeaderValue>,

    /// The request's extensions
    pub extensions: Extensions,

}