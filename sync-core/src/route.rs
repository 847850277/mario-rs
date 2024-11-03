pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn() -> String,
}

impl Route {
    pub fn new(method: String, path: String, handler: fn() -> String) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
