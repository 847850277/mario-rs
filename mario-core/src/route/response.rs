pub struct Response {
    body: String,
}

impl Response {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
        }
    }
}
