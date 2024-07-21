use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;

/// Healthcheck handler
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
   let server = HttpServer::new(|| {
        App::new().route("/healthcheck", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}