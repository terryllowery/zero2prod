use std::net::TcpListener;

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

fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port().to_string();
    let server = zero2prod::run(listner).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
