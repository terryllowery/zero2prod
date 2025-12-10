use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

// TODO: implement actual subscription logic
pub async fn subscribe(
    _form: web::Form<FormData>,
    _db_connection: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        _form.email,
        _form.name,
        chrono::Utc::now()
    )
    .execute(_db_connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}
