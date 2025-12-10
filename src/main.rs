use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", _configuration.application_port);
    let db_connection_pool = PgPool::connect(&_configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database.");

    let listener = TcpListener::bind(address)?;
    run(listener, db_connection_pool)?.await
}
