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

pub fn proxy_url(request: Request, url: &str) {
    let target_url = build_target_url(url);
    match ureq::get(&target_url).call() {
        Ok(resp) => {
            // Build a tiny_http::Response with the same status and headers
            let status = resp.status();
            let mut response = Response::empty(status);
            // Copy headers
            for header in resp.headers_names() {
                if let Some(value) = resp.header(&header) {
                    response.add_header(
                        tiny_http::Header::from_bytes(header.as_bytes(), value.as_bytes())
                            .unwrap_or_else(|_| {
                                // fallback: skip invalid headers
                                tiny_http::Header::from_bytes(b"X-Invalid-Header", b"invalid")
                                    .unwrap()
                            }),
                    );
                }
            }
            // Copy body
            let mut body = Vec::new();
            let _ = resp.into_reader().read_to_end(&mut body);
            let mut response_with_body = Response::from_data(body);
            // Copy headers again to the new response
            for header in response.headers() {
                response_with_body.add_header(header.clone());
            }
            let _ = request.respond(response_with_body);
        }
        Err(_) => {
            let response =
                Response::from_string("Failed to fetch target URL").with_status_code(502);
            let _ = request.respond(response);
        }
    }
}
