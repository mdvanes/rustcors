mod handler;
mod proxy;
mod usage;
mod not_found;

use std::env;
use tiny_http::{Server};

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).unwrap();
    println!("Server running on http://{}/", addr);
    
    for request in server.incoming_requests() {
        handler::handle_request(request);
    }
}
