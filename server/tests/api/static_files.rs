// tests/api/static_files.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server};

#[tokio::test]
async fn index_route_returns_200_and_static_files() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/index.html", addr))
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await.expect("Failed to read response body");

    // Assert
    assert_eq!(status, 200);

    let content_type = headers
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();
    assert_eq!(content_type, "text/html");

    assert!(
        body.contains("<html"),
        "Expected HTML content, got: {}",
        body
    );
}
