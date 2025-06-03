# RustCORS

A CORS Proxy in Rust using [tiny_http](https://crates.io/crates/tiny_http).

## Running the server

```sh
cargo run
```

The server will start on http://0.0.0.0:8080/

- GET / → returns `Hello, World!`
- Any other path or method → returns 404 Not Found

## Dependencies
- [tiny_http](https://crates.io/crates/tiny_http)

## License
MIT
