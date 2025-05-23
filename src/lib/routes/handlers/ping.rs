// src/lib/routes/ping.rs

// dependencies
use crate::actors::AnalyticsMessage;
use crate::actors::ping::PingMessage;
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq, SvcResp};
use crate::utilities::{json_response_msg, set_content_type_json};
use hyper::Response;
use matchit::Params;
use tokio::sync::oneshot;

// ping handler function
pub fn handle_ping(_request: SvcReq, state: AppState, _params: Params) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Ping endpoint reached");

        state
            .ping_tx
            .send(PingMessage::Ping)
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let (tx, rx) = oneshot::channel();

        state
            .analytics_tx
            .send(AnalyticsMessage::Increment {
                key: "ping".to_string(),
                reply: Some(tx),
            })
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        rx.await.map_err(|_| ApiError::ActorFailed)?;

        let response_msg = "Pong";
        let mut response: SvcResp = Response::new(json_response_msg(response_msg));
        set_content_type_json(&mut response);

        Ok(response)
    })
}
