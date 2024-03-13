use std::net::TcpListener;

use zerotoprod;

#[tokio::test]
async fn health_check() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").expect("failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let server = zerotoprod::run(listener).expect("failed to bind to port");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
