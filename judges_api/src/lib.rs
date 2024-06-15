use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use axum::{
    extract::Extension,
    routing::{delete, get, patch, post},
    Router,
};

use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

pub mod handlers;
pub mod models;

pub fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to judges API!" }))
        .route("/judges", get(handlers::list_all_judges))
        .route("/judges/:id", get(handlers::get_judge))
        .route("/judges", post(handlers::create_judge))
        .route("/judges/:id", patch(handlers::update_judge))
        .route("/judges/:id", delete(handlers::delete_judge))
        .route("/judges/nomination_date_range", get(handlers::get_judges_by_nomination_date_range))
        .route("/judges/professional_career/:keyword", get(handlers::get_judges_by_professional_career))
        .route("/judges/court/:court_name", get(handlers::get_judges_by_court))
        .route("/judges/president/:president_name", get(handlers::get_judges_by_president))
        .route("/judges/aba_rating/:aba_rating", get(handlers::get_judges_by_aba_rating))
        .route("/judges/gender/:gender", get(handlers::get_judges_by_gender))
        .route("/judges/birth_year_range/:start_year/:end_year", get(handlers::get_judges_by_birth_year_range))
        .route("/judges/race_ethnicity/:race_ethnicity", get(handlers::get_judges_by_race_ethnicity))
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner())
        .layer(CorsLayer::permissive())
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3018));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
