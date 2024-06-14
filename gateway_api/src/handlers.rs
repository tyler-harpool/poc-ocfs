use axum::{http::StatusCode, response::Json};
use reqwest::Client;
use serde_json::{json, Value};

pub async fn root() -> Json<Value> {
    Json(json!({ "message": "Welcome to the API Gateway" }))
}

async fn forward_request(path: &str, port: u16) -> Result<Json<Value>, StatusCode> {
    let client = Client::new();
    let url = format!("http://localhost:{}{}", port, path); // Adjust the URL as needed
    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if res.status().is_success() {
        let json = res
            .json::<Value>()
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        Ok(Json(json))
    } else {
        Err(StatusCode::BAD_GATEWAY)
    }
}

pub async fn forward_adr() -> Result<Json<Value>, StatusCode> {
    forward_request("/", 8000).await
}


pub async fn forward_attorney_adr() -> Result<Json<Value>, StatusCode> {
    forward_request("/adr", 3000).await
}
pub async fn forward_attorney_advocate() -> Result<Json<Value>, StatusCode> {
    forward_request("/attorney_advocates", 3001).await
}

pub async fn forward_case_data() -> Result<Json<Value>, StatusCode> {
    forward_request("/case_data", 3002).await
}

pub async fn forward_charges() -> Result<Json<Value>, StatusCode> {
    forward_request("/charges", 3003).await
}

pub async fn forward_civil_judgments() -> Result<Json<Value>, StatusCode> {
    forward_request("/civil_judgments", 3004).await
}

pub async fn forward_dependency_permanency() -> Result<Json<Value>, StatusCode> {
    forward_request("/dependencyPermanency", 3005).await
}

pub async fn forward_diversion() -> Result<Json<Value>, StatusCode> {
    forward_request("/diversions", 3006).await
}

pub async fn forward_hearings_events() -> Result<Json<Value>, StatusCode> {
    forward_request("/hearings_events", 3007).await
}

pub async fn forward_motions_filings() -> Result<Json<Value>, StatusCode> {
    forward_request("/motions_filings", 3008).await
}

pub async fn forward_orders() -> Result<Json<Value>, StatusCode> {
    forward_request("/orders", 3009).await
}

pub async fn forward_participants() -> Result<Json<Value>, StatusCode> {
    forward_request("/participants", 3010).await
}

pub async fn forward_pleadings() -> Result<Json<Value>, StatusCode> {
    forward_request("/pleadings", 3011).await
}


pub async fn forward_post_trial() -> Result<Json<Value>, StatusCode> {
    forward_request("/postTrial", 3012).await
}

pub async fn forward_pretrial_intake() -> Result<Json<Value>, StatusCode> {
    forward_request("/pretrial_intake", 3013).await
}

pub async fn forward_probate() -> Result<Json<Value>, StatusCode> {
    forward_request("/probateReviewMonitor", 3014).await
}

pub async fn forward_sanctions() -> Result<Json<Value>, StatusCode> {
    forward_request("/sanctions", 3015).await
}

pub async fn forward_status() -> Result<Json<Value>, StatusCode> {
    forward_request("/status", 3016).await
}

pub async fn forward_pdf() -> Result<Json<Value>, StatusCode> {
    forward_request("/ocr", 3017).await
}

pub async fn forward_judges() -> Result<Json<Value>, StatusCode> {
    forward_request("/judges", 3018).await
}