use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

// TODO: implement actual subscription logic
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
