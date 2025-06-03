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
            let mut body = String::new();
            let _ = resp.into_reader().read_to_string(&mut body);
            let response = Response::from_string(body);
            let _ = request.respond(response);
        }
        Err(_) => {
            let response =
                Response::from_string("Failed to fetch target URL").with_status_code(502);
            let _ = request.respond(response);
        }
    }
}
