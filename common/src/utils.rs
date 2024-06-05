use axum::http;

pub fn log_request(headers: &http::HeaderMap, id: Option<i32>, action: &str) {
    let id_str = id.map_or("unknown".to_string(), |id| id.to_string());
    if let Some(header_value) = headers.get("X-Test-Client") {
        tracing::info!("<-- TEST -- Request from test client: {:?}", header_value);
        tracing::info!("<-- TEST -- {} with id: {}", action, id_str);
    } else {
        tracing::info!("{} with id: {}", action, id_str);
    }
}
