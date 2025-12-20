use std::fmt::format;
use sqlx::{Executor, PgConnection, PgPool};
use sqlx::Connection;
use std::net::TcpListener;
use sqlx::Error::Database;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let maint_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "postgres".to_string(),
        ..config.clone()
    };
    let mut connection = PgConnection::connect(&maint_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    connection.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    connection.execute(format!(r#"ALTER DATABASE "{}" OWNER TO "{}";"#, config.database_name, config.username).as_str())
        .await
        .expect("Failed to alter database.");

    connection.execute(format!(r#"GRANT ALL PRIVILEGES ON DATABASE "{}" TO "{}";"#, config.database_name, config.username).as_str())
        .await
        .expect("Failed to grant privileges on database.");

    let maint_settings_for_new_db = DatabaseSettings {
        database_name: config.database_name.clone(),
        username: config.username.clone(),
        password: config.password.clone(),
        ..config.clone()
    };

    let mut new_db_conn = PgConnection::connect(&maint_settings_for_new_db.connection_string())
        .await
        .expect("Failed to connect to the newly created Postgres database as superuser");



    new_db_conn.execute(format!(r#"GRANT ALL ON SCHEMA public TO "{}";"#, config.username).as_str())
        .await
        .expect("Failed to grant privileges on schema.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

/// Spin up instance of our application
/// and returns it's address
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to create TCP listener");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration =
        get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let db_pool = configure_database(&configuration.database)
        .await;

    let server = zero2prod::startup::run(listener, db_pool.clone()).expect("Failed to start server");
    let _ = tokio::spawn(server);
    TestApp { address, db_pool }
}
#[tokio::test]
async fn health_check_works() {
    let  app= spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();



    // Act
    let body = "name=le%20guin&email=ursula_le_quin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(
        "SELECT email, name FROM subscriptions WHERE email = $1 ",
        "ursula_le_quin@gmail.com"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_quin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let app_address = &app.address;

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_quin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 bad Request when the payload was {}.",
            error_message
        );
    }
}
