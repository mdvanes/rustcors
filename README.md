# RustCORS 🦀

A tiny CORS Anywhere proxy made with [Rust](https://www.rust-lang.org/).

Adds CORS headers to each request to be able to call APIs that require CORS, without setting up your own server. Just start up this Docker container and start sending requests.

All requests to this proxy are allowed with this header: Access-Control-Allow-Origin: \*! and proxied to the supplied URL. Also see CORS Anywhere and enable-cors.org.

## Running the server

```sh
PORT=5001 cargo run
```

The server will start on http://0.0.0.0:5001/

- GET http://0.0.0.0:5001/https://mdworld.nl → returns the content of https://mdworld.nl. This works fine for JSON content. If the content is HTML, it will render the HTML but will not resolve resources like JS, CSS, or images that are imported with a relative path.
- When the `PORT` envar is left out, the default port is 5080

## Build

cargo test
cargo build --release
PORT=5000 ./target/release/rustcors
