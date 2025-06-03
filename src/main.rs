use std::env;
use tiny_http::{Server, Response};
use std::io::Read;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).unwrap();
    println!("Server running on http://{}/", addr);
    for request in server.incoming_requests() {
        if request.method() == &tiny_http::Method::Get {
            let url = request.url();
            if url.starts_with("/") && url.len() > 1 {
                // Remove the leading '/'
                let param = &url[1..];
                // Try to parse as a URL, add scheme if missing
                let target_url = if param.starts_with("http://") || param.starts_with("https://") {
                    param.to_string()
                } else if param.contains(":443") {
                    format!("https://{}", param.trim_end_matches(":443"))
                } else {
                    format!("http://{}", param)
                };
                match ureq::get(&target_url).call() {
                    Ok(mut resp) => {
                        let mut body = String::new();
                        let _ = resp.into_reader().read_to_string(&mut body);
                        let response = Response::from_string(body);
                        let _ = request.respond(response);
                    },
                    Err(_) => {
                        let response = Response::from_string("Failed to fetch target URL").with_status_code(502);
                        let _ = request.respond(response);
                    }
                }
            } else {
                let usage = "Usage:\n\
GET /<url> - Proxies the supplied URL and adds CORS headers.\n\
Examples:\n\
  /http://mdworld.nl/   → proxies http://mdworld.nl/\n\
  /mdworld.nl:443       → proxies https://mdworld.nl/\n\
  /                     → shows this help message\n";
                let response = Response::from_string(usage);
                let _ = request.respond(response);
            }
        } else {
            let response = Response::from_string("Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}
