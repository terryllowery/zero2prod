use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::PgPool;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    // TODO: Remove this debug print
    println!("{:?}", configuration);
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    let connection = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres.");

    run(listener, connection)?.await
}
