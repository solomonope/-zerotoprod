use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zerotoprod::{
    self,
    configuration::{self, get_configurations},
};

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();

    let configuration = get_configurations().expect("failed to read configuration");

    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("failed to connect to postgres");

    let client = reqwest::Client::new();

    let body = "name=opeyemi&email=solomonope@gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=opeyemi", "missing the email"),
        ("email=solomonope@gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        )
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").expect("failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let server = zerotoprod::run(listener).expect("failed to bind to port");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
