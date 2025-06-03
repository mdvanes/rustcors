use tiny_http::{Request, Response};

pub fn respond_not_found(request: Request) {
    log!("[respond_not_found] Responding with 404 Not Found");
    let response = Response::from_string("Not Found").with_status_code(404);
    let _ = request.respond(response);
}

pub fn respond_forbidden(request: Request) {
    log!("[respond_forbidden] Responding with 403 Forbidden");
    let response = Response::from_string("Forbidden").with_status_code(403);
    let _ = request.respond(response);
}

pub fn respond_usage(request: Request) {
    log!("[respond_usage] Responding with usage information");
    let usage = "Usage:\n\
GET /<url> - Proxies the supplied URL and adds CORS headers.\n\
Examples:\n\
  /http://mdworld.nl/   → proxies http://mdworld.nl/\n\
  /mdworld.nl:443       → proxies https://mdworld.nl/\n\
  /                     → shows this help message\n";
    let response = Response::from_string(usage);
    let _ = request.respond(response);
}
