use tiny_http::{Request, Response};

pub fn respond_usage(request: Request) {
    let usage = "Usage:\n\
GET /<url> - Proxies the supplied URL and adds CORS headers.\n\
Examples:\n\
  /http://mdworld.nl/   → proxies http://mdworld.nl/\n\
  /mdworld.nl:443       → proxies https://mdworld.nl/\n\
  /                     → shows this help message\n";
    let response = Response::from_string(usage);
    let _ = request.respond(response);
}
