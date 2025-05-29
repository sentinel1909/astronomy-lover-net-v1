// tests/api/ping.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server, start_test_server_with_state};
use astronomy_lover_net_server_lib::actors::{
    AnalyticsMessage, FetchMessage, FilesMessage, PingMessage,
};
use astronomy_lover_net_server_lib::init::build_route_table;
use astronomy_lover_net_server_lib::state::AppState;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Deserialize)]
struct TestResponse<T> {
    msg: String,
    content: T,
}

#[tokio::test]
async fn ping_route_returns_200_ok() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/ping", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 200);

    let ping_response: TestResponse<String> = response
        .json()
        .await
        .expect("Failed to parse JSON from /count");

    // Assert: count is 3
    assert_eq!(ping_response.msg, "success");
    assert_eq!(ping_response.content, "Pong");
}

#[tokio::test]
async fn ping_route_returns_502_when_actor_dropped() {
    // Arrange
    let (analytics_tx, _analytics_rx) = mpsc::channel::<AnalyticsMessage>(1);
    let (fetch_tx, _fetch_rx) = mpsc::channel::<FetchMessage>(1);
    let (files_tx, _files_rx) = mpsc::channel::<FilesMessage>(1);
    let (ping_tx, ping_rx) = mpsc::channel::<PingMessage>(1);

    drop(ping_rx);

    let state = AppState {
        analytics_tx,
        fetch_tx,
        files_tx,
        ping_tx,
        routes: Arc::new(build_route_table()),
    };

    let addr = start_test_server_with_state(state).await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/ping", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 502);
}
