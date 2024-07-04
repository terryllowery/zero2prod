use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;

/// Healthcheck handler
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/healthcheck", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}