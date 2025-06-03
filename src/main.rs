mod handler;
mod not_found;
mod proxy;
mod usage;

use std::env;
use tiny_http::Server;

macro_rules! log {
    ($($arg:tt)*) => {{
        let now = chrono::Local::now().format("%Y/%m/%d %H:%M:%S");
        println!("{} {}", now, format!($($arg)*));
    }};
}

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "5080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).unwrap();

    let origins: Vec<String> = match env::var("ORIGINS") {
        Ok(val) => val.split(',').filter(|s| !s.trim().is_empty()).map(|s| s.trim().to_string()).collect(),
        Err(_) => Vec::new(),
    };

    log!("Allowed origins: {:?}", origins);

    log!("Starting RustCORS 🦀 server on http://{}/", addr);

    for request in server.incoming_requests() {
        handler::handle_request(request);
    }
}
