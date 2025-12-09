use actix_web::{App, HttpResponse, HttpServer, web};
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
