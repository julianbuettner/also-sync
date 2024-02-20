#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use also_sync_macros::also_sync_tokio;
#[cfg(feature = "tokio")]
use lazy_static::lazy_static;
#[cfg(feature = "tokio")]
use tokio::runtime::{Builder, Runtime};

#[cfg(feature = "tokio")]
lazy_static! {
    /// This is the global tokio runtime in which all `*_sync` functions
    /// are executed.
    pub static ref TOKIO_RUNTIME: Runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

#[cfg(test)]
mod test {
    use crate as also_sync;

    #[also_sync_tokio]
    async fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    struct WithSelf;

    impl WithSelf {
        #[also_sync_tokio]
        pub async fn with_self(self, a: i32) -> i32 {
            a
        }
    }
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_sync(2, 3), 5);
    }

    #[test]
    fn test_self() {
        let w = WithSelf {};
        assert_eq!(w.with_self_sync(42), 42)
    }
}
