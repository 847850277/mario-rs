use super::{FailedToDeserializeQueryString, FromRequest, QueryRejection, RequestParts};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::ops::Deref;

/// Extractor that deserializes query strings into some type.
///
/// If the query string cannot be parsed it will reject the request with a `400
/// Bad Request` response.
#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Query<T>
where
    T: DeserializeOwned,
    B: Send,
{
    type Rejection = QueryRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let query = req.uri().query().unwrap_or_default();
        let value = serde_urlencoded::from_str(query)
            .map_err(FailedToDeserializeQueryString::new::<T, _>)?;
        Ok(Query(value))
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
