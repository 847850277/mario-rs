pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    inner: BoxError,
}

impl Error {
    pub(crate) fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: error.into(),
        }
    }
}
