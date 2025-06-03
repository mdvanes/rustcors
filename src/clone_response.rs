use std::io::Read;
use tiny_http::Response;

pub fn clone_response(resp: ureq::Response) -> Response<std::io::Cursor<Vec<u8>>> {
    // Build a tiny_http::Response with the same status and headers
    let status = resp.status();
    let mut response = Response::empty(status);

    // Copy headers
    for header in resp.headers_names() {
        if let Some(value) = resp.header(&header) {
            response.add_header(
                tiny_http::Header::from_bytes(header.as_bytes(), value.as_bytes()).unwrap_or_else(
                    |_| {
                        // fallback: skip invalid headers
                        tiny_http::Header::from_bytes(b"X-Invalid-Header", b"invalid").unwrap()
                    },
                ),
            );
        }
    }

    // Copy body
    let mut body = Vec::new();
    let _ = resp.into_reader().read_to_end(&mut body);
    let mut response_with_body = Response::from_data(body);

    // After resp.into_reader it's not possible to read the resp.headers, so copy them from response
    for header in response.headers() {
        response_with_body.add_header(header.clone());
    }

    // Set the status code on the response with body
    let response_with_body = response_with_body.with_status_code(status);

    response_with_body
}
