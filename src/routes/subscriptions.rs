use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
/// Subscribe to the newsletter
pub(crate) async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
   match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id , email, name, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.as_ref())
    .await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}