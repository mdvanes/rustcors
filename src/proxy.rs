use super::add_cors_headers::add_cors_headers;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_target_url_http() {
        assert_eq!(build_target_url("http://example.com"), "http://example.com");
    }

    #[test]
    fn test_build_target_url_https() {
        assert_eq!(
            build_target_url("https://example.com"),
            "https://example.com"
        );
    }

    #[test]
    fn test_build_target_url_443() {
        assert_eq!(build_target_url("example.com:443"), "https://example.com");
    }

    #[test]
    fn test_build_target_url_plain() {
        assert_eq!(build_target_url("example.com"), "http://example.com");
    }
}
