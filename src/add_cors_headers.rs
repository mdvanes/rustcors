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

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_http::Response;

    #[test]
    fn adds_cors_headers() {
        let mut response = Response::from_data(Vec::new());
        add_cors_headers(&mut response);
        let mut found_origin = false;
        let mut found_headers = false;
        for header in response.headers() {
            if header.field.equiv("Access-Control-Allow-Origin") && header.value == "*" {
                found_origin = true;
            }
            if header.field.equiv("Access-Control-Allow-Headers") && header.value == "Origin, X-Requested-With, Content-Type, Accept"
            {
                found_headers = true;
            }
        }
        assert!(found_origin, "Access-Control-Allow-Origin header not found or incorrect");
        assert!(found_headers, "Access-Control-Allow-Headers header not found or incorrect");
    }
}