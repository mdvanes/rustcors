use tiny_http::{Request, Response};

pub fn respond_not_found(request: Request) {
    let response = Response::from_string("Not Found").with_status_code(404);
    let _ = request.respond(response);
}
