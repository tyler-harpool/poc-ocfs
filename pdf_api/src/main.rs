use actix_web::{web::{self}, App, HttpServer};
mod handlers;
mod models;
use handlers::{upload_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .route("/upload", web::post().to(upload_file))
            .route("/ocr", web::post().to(upload_file))

    })
        .bind("127.0.0.1:3017")?
        .run()
        .await
}