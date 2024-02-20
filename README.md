# Also Sync

This library provides a macro to automatically derive a
`*_sync` version from your async functions. It does so
by having a global tokio runtime with all features
on all cores.

This allows a async-first library development while
supporting sync.

## Notes and considerations
- Enabling `tokio` in `also_async` adds the entire
  tokio runtime as a dependency and creates a runtime
  instance at startup.
- If the library is used in an async runtime, the sync
  function is ran in a different runtime. This
  might change in the future by making the async
  functions private if a sync feature is enabled.
- This library is currently more a proof of concept than a serious
  solution to the problem.
- Currently only the tokio runtime is available to execute async tasks.

## How to make your crate (lib) also sync

```toml
# Cargo.toml
[package]
# ...

[features]
# If also_sync is enabled, it enables tokio in also_sync
also_sync = ["also_sync/tokio"]

[dependencies]
# Disable the tokio default, to only use it if enabled
# by the dependee
also_sync = { version = "0.1", default-features = false }

```

```rs
// lib.r
use also_sync::also_sync_tokio;

pub struct MyApi;

impl MyApi {
    // If also_sync/tokio is not enabled, this macro
    // is no-op.
    #[also_sync_tokio]
    pub async fn get_value(&self) -> i32 {
        42
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let api = MyApi {};
        assert_eq!(api.get_value_sync(), 42);
    }
}
```

## Run examples
```bash
cargo run --example weather
cargo run --example dns_lookups
```


## How does it work?
Take the following code as an example:
```rs
use also_sync::also_sync;

struct Struct;

impl Struct {
    #[also_sync]
    pub async fn foo(&self, a: i32) -> i32 {
        42 * a
    }
}
```

This is literally just converted to
```rs
struct Struct;

impl Struct {
    pub async fn foo(&self, a: i32) -> i32 {
        42
    }
    pub fn foo_sync(&self: a: i32) -> i32 {
        let handle = also_sync::TOKIO_RUNTIME.handle();
        handle.block_on(async move { self.foo(a).await })
    }
}
```

Of course if works for simple functions as well.

