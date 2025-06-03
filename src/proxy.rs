use std::io::Read;
use tiny_http::{Request, Response};

fn build_target_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else if url.contains(":443") {
        format!("https://{}", url.trim_end_matches(":443"))
    } else {
        format!("http://{}", url)
    }
}

fn add_cors_headers<R: std::io::Read>(response: &mut tiny_http::Response<R>) {
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

fn clone_response(resp: ureq::Response) -> Response<std::io::Cursor<Vec<u8>>> {
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

pub fn proxy_url(request: Request, url: &str) {
    let target_url = build_target_url(url);
    match ureq::get(&target_url).call() {
        Ok(resp) => {
            let mut response_with_body = clone_response(resp);

            // Add CORS headers
            add_cors_headers(&mut response_with_body);

            let _ = request.respond(response_with_body);
        }
        Err(e) => {
            println!(
                "[proxy_url] Failed to fetch target URL '{}': {}",
                target_url, e
            );
            let response =
                Response::from_string("Failed to fetch target URL").with_status_code(502);
            let _ = request.respond(response);
        }
    }
}
