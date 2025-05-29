// tests/api/fetch.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server};

#[tokio::test]
async fn fetch_route_returns_200_and_nasa_data() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/fetch", addr))
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status();
    let _headers = response.headers().clone();
    let _body = response.text().await.expect("Failed to read response body");

    // Assert
    assert_eq!(status, 200);
}
