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
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id: {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    log::info!("request_id: {} - Adding a new subscriber.", request_id);
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
            log::info!("request_id: {} - Subscriber added successfully.", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request_id: {} - Failed to execute query: {:?}",request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

}