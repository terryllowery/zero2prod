//! tests/health_check.rs
use std::net::TcpListener;
use zero2prod::startup::run;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

/// This test checks if the health check endpoint is working
#[tokio::test]
async fn health_check_works() {
    // Arrange
    // let url = spawn_app();
    let url = format!("{}/healthcheck", spawn_app());
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
    let app_address = spawn_app();
    let config = get_configuration().expect("Failed to read configuration.");
    let connection_string = config.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to Postgres.");


    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=urseula_le_guin@gmail.com";
    let response = client.post(&format!("{}/subscriptions", &app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&connection)
    .await
    .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

}

/// subscribe returns a 400 when data is missing
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missin() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=urseula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    // Act

    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &app_address))
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
fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port().to_string();
    let server = run(listner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
