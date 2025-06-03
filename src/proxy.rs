use super::clone_response::clone_response;
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
