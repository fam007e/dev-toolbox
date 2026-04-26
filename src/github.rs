use reqwest::header::HeaderMap;

pub fn check_token_scopes(headers: &HeaderMap) -> Option<String> {
    if let Some(scopes_header) = headers.get("x-oauth-scopes") {
        let scopes = scopes_header.to_str().unwrap_or("");
        let scope_list: Vec<&str> = scopes.split(',').map(|s| s.trim()).collect();
        
        let has_dangerous = scope_list.iter().any(|&s| {
            s == "repo" || s == "delete_repo" || s.starts_with("admin:") || s.starts_with("write:")
        });

        if has_dangerous {
            return Some(format!("Security Warning: Token has broad scopes: {}. Consider using a fine-grained token with read-only access.", scopes));
        }
    }
    None
}
