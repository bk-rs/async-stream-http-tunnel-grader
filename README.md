# async-stream-http-tunnel-grader

* [Cargo package](https://crates.io/crates/async-stream-http-tunnel-grader)

## Examples

### smol 

* [async_http1_lite_client](demos/smol/src/async_http1_lite_client.rs)

## Dev

```
cargo test --all-features --all -- --nocapture && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features --all
```

```
cargo tarpaulin --all-features
```
