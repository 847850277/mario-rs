pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn(),
}

impl Route {
    pub fn new(method: String, path: String, handler: fn()) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
