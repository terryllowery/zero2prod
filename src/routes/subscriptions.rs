use actix_web::{web, HttpResponse};
use sqlx::PgConnection;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
/// Subscribe to the newsletter
pub(crate) async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgConnection>) -> HttpResponse {
    // TODO: Fix this using pgPool
    sqlx::query!(
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
    .await;
    HttpResponse::Ok().finish()
}