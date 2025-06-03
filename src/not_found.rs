use tiny_http::{Request, Response};
// use super::logger::log;

pub fn respond_not_found(request: Request) {
    let response = Response::from_string("Not Found").with_status_code(404);
    let _ = request.respond(response);
}

pub fn respond_forbidden(request: Request) {
    // log("[respond_forbidden] Responding with 403 Forbidden");
    let response = Response::from_string("Forbidden").with_status_code(403);
    let _ = request.respond(response);
}
