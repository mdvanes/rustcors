use tiny_http::Request;

pub fn check_origin_header(request: &Request, origins: &Vec<String>) -> bool {
    let origin_header = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Origin"))
        .map(|h| h.value.as_str());
    is_origin_allowed(origin_header, origins)
}

pub fn is_origin_allowed(origin: Option<&str>, origins: &Vec<String>) -> bool {
    if origins.is_empty() {
        log!("[check_origin_header] No allowed origins configured, allowing all origins");
        return true;
    }
    if let Some(origin_value) = origin {
        origins.iter().any(|o| o == origin_value)
    } else {
        log!(
            "[check_origin_header] No Origin header present. Allowed list: {:?}",
            origins
        );
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_all_when_origins_empty() {
        let origins = vec![];
        assert!(is_origin_allowed(Some("https://foo.com"), &origins));
    }

    #[test]
    fn denies_when_origin_not_in_list() {
        let origins = vec!["https://allowed.com".to_string()];
        assert!(!is_origin_allowed(Some("https://notallowed.com"), &origins));
    }

    #[test]
    fn allows_when_origin_in_list() {
        let origins = vec!["https://allowed.com".to_string()];
        assert!(is_origin_allowed(Some("https://allowed.com"), &origins));
    }
}
