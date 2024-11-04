use http::Response;

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn() -> Response<String>,
}

impl Route {
    pub fn new(method: String, path: String, handler: fn() -> Response<String>) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
