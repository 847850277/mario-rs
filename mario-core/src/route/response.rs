//#[derive(Debug, Clone)]
pub struct Response<T> {
    body: T,
}

impl<T: std::fmt::Debug> Response<T> {
    fn as_text(&self) -> String {
        return format!("{:?}", self.body);
    }
}

impl<T> Response<T> {
    pub fn new(body: T) -> Response<T> {
        Response { body }
    }
    pub fn set_body(&mut self, body: T) {
        self.body = body;
    }

    pub fn get_body(&self) -> &T {
        return &self.body;
    }
}
