use tiny_http::Request;

use super::check_origin_header::check_origin_header;
use super::proxy::proxy_url;
use super::static_responses::{respond_forbidden, respond_not_found, respond_usage};

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
