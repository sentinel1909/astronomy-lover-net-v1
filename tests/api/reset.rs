// tests/api/analytics_reset.rs

use crate::helpers::{get_test_client, start_test_server};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct TestResponse<T> {
    msg: String,
    content: T,
}

#[tokio::test]
async fn metrics_reset_clears_all_data() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act 1: Trigger some analytics data by hitting /ping multiple times
    for _ in 0..5 {
        let ping_resp = client
            .get(format!("http://{}/ping", addr))
            .send()
            .await
            .expect("Failed to call /ping");
        assert_eq!(ping_resp.status(), 200);
    }

    // Act 2: Verify metrics show the ping count
    let metrics_resp = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Failed to call /metrics");
    assert_eq!(metrics_resp.status(), 200);

    let metrics_body: TestResponse<HashMap<String, usize>> = metrics_resp
        .json()
        .await
        .expect("Failed to parse metrics response");
    assert_eq!(metrics_body.msg, "success");
    assert_eq!(metrics_body.content.get("ping"), Some(&5));

    // Act 3: Reset the metrics
    let reset_resp = client
        .post(format!("http://{}/metrics/reset", addr))
        .send()
        .await
        .expect("Failed to call /metrics/reset");
    assert_eq!(reset_resp.status(), 200);

    let reset_body: TestResponse<String> = reset_resp
        .json()
        .await
        .expect("Failed to parse reset response");
    assert_eq!(reset_body.msg, "success");
    assert_eq!(reset_body.content, "metrics reset");

    // Act 4: Verify metrics are cleared
    let metrics_resp_after_reset = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Failed to call /metrics after reset");
    assert_eq!(metrics_resp_after_reset.status(), 200);

    let metrics_body_after_reset: TestResponse<HashMap<String, usize>> = metrics_resp_after_reset
        .json()
        .await
        .expect("Failed to parse metrics after reset");
    assert!(metrics_body_after_reset.content.is_empty());
}
