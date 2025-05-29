// src/lib/routes/fetch.rs

// dependencies
use crate::actors::{FetchMessage, FetchResult};
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq};
use crate::utilities::json_response_msg;
use hyper::Response;
use matchit::Params;
use tokio::sync::oneshot;

// health check handler function
pub fn handle_fetch_nasa_data(_request: SvcReq, state: AppState, _params: Params) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Fetch handler reached");

        let (tx, rx) = oneshot::channel();
        state
            .fetch_tx
            .send(FetchMessage::Get { reply: tx })
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let fetch_result = rx.await.map_err(|_| ApiError::ActorFailed)?;

        match fetch_result {
            FetchResult::Ok(nasa_data) => Ok(Response::new(json_response_msg(nasa_data))),
            FetchResult::FetchError(e) => Err(ApiError::NetworkError(e)),
        }
    })
}
