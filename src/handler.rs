use tiny_http::Request;

use super::not_found::respond_not_found;
use super::proxy::proxy_url;
use super::usage::respond_usage;

pub fn handle_request(request: Request) {
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
