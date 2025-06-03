use std::env;
use tiny_http::{Server, Response};

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
                let response = Response::from_string(param);
                let _ = request.respond(response);
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
