use http::request::Parts;
use http::{Extensions, HeaderMap, HeaderValue, Method, Uri, Version};
use hyper::body::Incoming;

#[derive(Debug)]
pub struct Request {
    pub head: Head,
    pub body: Incoming,
}

impl Request {
    pub fn new(request: http::Request<hyper::body::Incoming>) -> Self {
        let (
            Parts {
                method,
                uri,
                version,
                headers,
                extensions,
                ..
            },
            body,
        ) = request.into_parts();

        Self {
            head: Head {
                method,
                uri: uri.clone(),
                version,
                headers,
                extensions,
            },
            body,
        }
    }
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

impl Clone for Head {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            uri: self.uri.clone(),
            version: self.version.clone(),
            headers: self.headers.clone(),
            extensions: self.extensions.clone(),
        }
    }
}
