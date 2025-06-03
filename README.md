# RustCORS

A tiny CORS Anywhere proxy made with Rust.

Adds CORS headers to each request to be able to call APIs that require CORS, without setting up your own server. Just start up this Docker container and start sending requests.

All requests to this proxy are allowed with this header: Access-Control-Allow-Origin: *! and proxied to the supplied URL. Also see CORS Anywhere and enable-cors.org.

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

# TODO

originAllowlist

http://localhost:5000/http://mdworld.nl/ - mdworld.nl with CORS headers
http://localhost:5000/mdworld.nl - Same as previous.
http://localhost:5000/mdworld.nl:443 - Proxies https://mdworld.nl/
http://localhost:5000/ - Shows usage text, as defined in lib/help.txt
http://localhost:5000/favicon.ico - Replies 404 Not found
