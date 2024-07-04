use actix_web::{App, HttpResponse, HttpServer, Responder, web};



/// Healthcheck handler
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(healthcheck))
    })
            .bind("127.0.0.1:8000")?
            .run()
            .await
    }
