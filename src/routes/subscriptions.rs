use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::Instrument;
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
    let request_span = tracing::info_span!(
        "adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    // TODO: remove commented code
    // tracing::info!(
    //     "request_id: {} - Adding '{}' '{}' as a new subscriber.",
    //     request_id,
    //     form.email,
    //     form.name
    // );
    // tracing::info!("request_id: {} - Adding a new subscriber.", request_id);
    let query_span = tracing::info_span!(
        "Saving new subscriber to the database",
    );
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
    .instrument(query_span)
    .await {
        Ok(_) => {
            tracing::info!("request_id: {} - Subscriber added successfully.", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id: {} - Failed to execute query: {:?}",request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

}