use crate::error::{BoxError, Error};
use bytes::Bytes;
use std::convert::Infallible;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

#[derive(Debug)]
pub struct FailedToDeserializeQueryString {
    error: Error,
    type_name: &'static str,
}

impl FailedToDeserializeQueryString {
    pub fn new<T, E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        FailedToDeserializeQueryString {
            error: Error::new(error),
            type_name: std::any::type_name::<T>(),
        }
    }
}
