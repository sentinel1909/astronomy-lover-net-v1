// src/lib/routes/index.rs

// dependencies
use crate::actors::{AnalyticsMessage, FilesMessage, FilesResult};
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq};
use crate::utilities::bytes_response_msg;
use matchit::Params;
use tokio::sync::oneshot;

// health check handler function
pub fn handle_static_files(_request: SvcReq, state: AppState, params: Params) -> HandlerResult {
    let path = params.get("path").unwrap_or_default().to_string();

    Box::pin(async move {
        tracing::info!("Static files handler reached");

        let (tx, rx) = oneshot::channel();
        state
            .files_tx
            .send(FilesMessage::Get {
                path: path.clone(),
                reply: tx,
            })
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let (files_result, full_path) = rx.await.map_err(|_| ApiError::ActorFailed)?;

        match files_result {
            FilesResult::Ok(bytes) => {
                tracing::info!("📂 Serving path: {}", path);

                if let Err(e) = state
                    .analytics_tx
                    .send(AnalyticsMessage::Increment {
                        key: path.clone(),
                        reply: None,
                    })
                    .await
                {
                    tracing::warn!("Failed to send analytics increment: {}", e);
                }
                Ok(bytes_response_msg(bytes, full_path.to_str().unwrap()))
            }
            FilesResult::NotFound => Err(ApiError::NotFound),
            FilesResult::IoError(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(ApiError::FileNotFound),
                _ => Err(ApiError::FileIo(e)),
            },
        }
    })
}
