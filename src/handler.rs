use tiny_http::Request;

use super::proxy::proxy_url;
use super::static_responses::{respond_forbidden, respond_not_found, respond_usage};

fn check_origin_header(request: &Request, origins: &Vec<String>) -> bool {
    let origin_header = request.headers().iter().find(|h| h.field.equiv("Origin"));

    if origins.is_empty() {
        log!("[check_origin_header] No allowed origins configured, allowing all origins");
        return true;
    }

    if let Some(origin_header) = origin_header {
        origins.iter().any(|o| o == origin_header.value.as_str())
    } else {
        log!(
            "[check_origin_header] Origin {:?} is not in the allowed list {:?}",
            origin_header,
            origins
        );
        false
    }
}

pub fn handle_request(request: Request, origins: &Vec<String>) {
    if !check_origin_header(&request, origins) {
        respond_forbidden(request);
        return;
    }

    if request.method() == &tiny_http::Method::Get {
        let url = request.url().to_string();
        if url.starts_with("/") && url.len() > 1 {
            // Remove the leading '/'
            let param = &url[1..];
            proxy_url(request, param);
        } else {
            respond_usage(request);
        }
    } else {
        respond_not_found(request);
    }
}
