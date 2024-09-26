#[derive(Debug,Clone)]
pub struct Response {
    body: String,
}

impl Response {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
        }
    }

    //get body
    pub fn body(&self) -> &str {
        &self.body
    }
}
