//#[derive(Debug, Clone)]
pub struct Response<T> {
    //body: String,
    body: T,
}


impl Response {
    // pub fn new(body: &str) -> Self {
    //     Self {
    //         body: body.to_string(),
    //     }
    // }

    //new




    //get body
    pub fn body(&self) -> &str {
        &self.body
    }
}
