use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Server running on http://0.0.0.0:8080/");
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
