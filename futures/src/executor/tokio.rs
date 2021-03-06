use crate::Executor;

use futures::Future;

/// A `tokio` runtime.
pub type Tokio = tokio::runtime::Runtime;

impl Executor for Tokio {
    fn new() -> Result<Self, futures::io::Error> {
        tokio::runtime::Runtime::new()
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let _ = tokio::runtime::Runtime::spawn(self, future);
    }

    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        tokio::runtime::Runtime::enter(self, f)
    }
}
