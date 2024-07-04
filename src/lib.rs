use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// Healthcheck handler
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/healthcheck", web::get().to(healthcheck)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
