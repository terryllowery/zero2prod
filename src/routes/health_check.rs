
use actix_web::HttpResponse;

/// Healthcheck handler
pub(crate) async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}