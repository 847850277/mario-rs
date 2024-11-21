pub use async_trait::async_trait;
pub use http;
pub use hyper::Server;
pub use tower_http::add_extension::{AddExtension, AddExtensionLayer};

#[macro_use]
mod macros;

mod body;
mod error;
pub mod router;

pub mod extract;
pub mod handler;
pub mod response;
mod util;
pub use self::router::Router;

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
