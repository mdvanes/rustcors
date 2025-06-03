pub fn add_cors_headers<R: std::io::Read>(response: &mut tiny_http::Response<R>) {
    response
        .add_header(tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"*").unwrap());
    response.add_header(
        tiny_http::Header::from_bytes(
            b"Access-Control-Allow-Headers",
            b"Origin, X-Requested-With, Content-Type, Accept",
        )
        .unwrap(),
    );
}