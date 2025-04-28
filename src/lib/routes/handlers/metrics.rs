// src/lib/routes/metrics.rs

// dependencies
use crate::actors::AnalyticsMessage;
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq, SvcResp};
use crate::utilities::{json_response_msg, set_content_type_json};
use hyper::Response;
use matchit::Params;
use tokio::sync::oneshot;

// metrics handler function
pub fn handle_metrics(_request: SvcReq, state: AppState, _params: Params) -> HandlerResult {
    Box::pin(async move {
        let (tx, rx) = oneshot::channel();

        state
            .analytics_tx
            .send(AnalyticsMessage::GetAll { reply: tx })
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let response_msg = rx.await.map_err(|_| ApiError::ActorFailed)?;

        let mut response: SvcResp = Response::new(json_response_msg(response_msg));
        set_content_type_json(&mut response);

        Ok(response)
    })
}

pub fn handle_reset_metrics(_req: SvcReq, state: AppState, _params: Params) -> HandlerResult {
    Box::pin(async move {
        state
            .analytics_tx
            .send(AnalyticsMessage::Reset)
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        Ok(Response::new(json_response_msg("metrics reset")))
    })
}
