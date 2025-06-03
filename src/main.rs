use std::env;
use tiny_http::{Server, Response};

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).unwrap();
    println!("Server running on http://{}/", addr);
    for request in server.incoming_requests() {
        if request.method() == &tiny_http::Method::Get && request.url() == "/" {
            let response = Response::from_string("Hello, World!");
            let _ = request.respond(response);
        } else {
            let response = Response::from_string("Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}
