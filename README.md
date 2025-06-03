# RustCORS

A CORS Proxy in Rust using [tiny_http](https://crates.io/crates/tiny_http).

## Running the server

```sh
PORT=5001 cargo run
```

The server will start on http://0.0.0.0:5001/

- GET / → returns `Hello, World!`
- Any other path or method → returns 404 Not Found

## Dependencies

- [tiny_http](https://crates.io/crates/tiny_http)

# Build

cargo build --release
PORT=5000 ./target/release/rustcors
