#[derive(Debug)]
pub struct Response<T> {
    body: T,
}

impl<T> Response<T> {
    pub fn new(body: T) -> Response<T> {
        Response { body }
    }
    pub fn set_body(&mut self, body: T) {
        self.body = body;
    }

    pub fn get_body(&self) -> &T {
        &self.body
    }
}
