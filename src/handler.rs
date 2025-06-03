use tiny_http::Request;

use super::not_found::{respond_forbidden, respond_not_found};
use super::proxy::proxy_url;
use super::usage::respond_usage;

fn check_origin_header(request: &Request, origins: &Vec<String>) -> bool {
    if let Some(origin_header) = request.headers().iter().find(|h| h.field.equiv("Origin")) {
        origins.iter().any(|o| o == origin_header.value.as_str())
    } else {
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
