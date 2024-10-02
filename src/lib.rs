use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;

/// Healthcheck handler
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

/// Subscribe to the newsletter
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
   let server = HttpServer::new(|| {
        App::new()
        .route("/healthcheck", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}