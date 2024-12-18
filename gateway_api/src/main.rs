use dotenv::dotenv;
use std::sync::Arc;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use gateway_api::{create_router, AppState};
use async_session::MemoryStore;
use reqwest::Client;
use anyhow::Context;
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let store = MemoryStore::new();
    let oauth_client = create_oauth_client().expect("Failed to create OAuth client");
    let http_client = Client::new();

    let app_state = Arc::new(AppState {
        store,
        oauth_client,
        http_client,
    });

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")
            .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

fn create_oauth_client() -> anyhow::Result<oauth2::basic::BasicClient> {
    let client_id = ClientId::new(
        std::env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."),
    );
    let client_secret = ClientSecret::new(
        std::env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );
    let redirect_url = RedirectUrl::new(
        std::env::var("REDIRECT_URL").unwrap_or_else(|_| "http://localhost:3000/auth/authorized".to_string()),
    )?;
    let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())?;
    let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".to_string())?;

    Ok(oauth2::basic::BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url))
}