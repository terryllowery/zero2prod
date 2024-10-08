//! tests/health_check.rs
use std::net::TcpListener;
use zero2prod::startup::run;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use zero2prod::configuration::{get_configuration, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// This test checks if the health check endpoint is working
#[tokio::test]
async fn health_check_works() {
    // Arrange
    // let url = spawn_app();
    let url = format!("{}/healthcheck", spawn_app().await.address);
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");


    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// subscribe resturns a 200 for valid form data
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin@gmail.com";
    let response = client.post(&format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

}

/// subscribe returns a 400 when data is missing
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missin() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=urseula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    // Act

    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

        // Assert
        assert_eq!(400, response.status().as_u16(), 
        "The API did not fail with 400 Bad Request when the payload was {}.", error_message);
        
    }

    // Assert
}
/// Spawns a new instance of our application
/// Returns the application address
async fn spawn_app() -> TestApp {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port().to_string();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to read configuration.");
    config.database.database_name = uuid::Uuid::new_v4().to_string();

    let connection_pool = configure_database(&config.database).await;

    let server = run(listner, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool
    }
}

/// Setup the test database
async fn configure_database(config: &DatabaseSettings) -> PgPool {

    // Create DB
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    )
    .await
    .expect("Failed to connect to Postgres.");


    connection
    .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
    .await
    .expect("Failed to create database.");

    // Migrate DB
    let connection_pool = PgPool::connect(
        &config.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database.");

    connection_pool

    

}

// TODO: Clean up the database after the test run