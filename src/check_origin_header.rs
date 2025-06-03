use tiny_http::Request;

pub fn check_origin_header(request: &Request, origins: &Vec<String>) -> bool {
    let origin_header = request.headers().iter().find(|h| h.field.equiv("Origin"));

    if origins.is_empty() {
        log!("[check_origin_header] No allowed origins configured, allowing all origins");
        return true;
    }

    if let Some(origin_header) = origin_header {
        origins.iter().any(|o| o == origin_header.value.as_str())
    } else {
        log!(
            "[check_origin_header] Origin {:?} is not in the allowed list {:?}",
            origin_header,
            origins
        );
        false
    }
}
