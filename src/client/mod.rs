//! HTTP Client implementation
//!
//! Currently we aim to provide only HTTP/1.x implementation. We want to
//! provide HTTP/2.0 and TLS implementation with exactly the same protocol.
//! But it's yet unproven if it is possible.
//!
//! Also DNS resolving is not implemented yet.

mod context;
mod pool;
mod request;
mod creator;

pub use self::context::{Context, Pool};
pub use self::request::{Request};
pub use self::creator::{RequestCreator};

trait PrivContext<R: RequestCreator> {
    fn pool(&mut self) -> &mut pool::Pool<R>;
}
