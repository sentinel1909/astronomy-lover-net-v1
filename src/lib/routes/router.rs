// src/lib/routes/router.rs

// dependencies

use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::{RouterResponse, SvcReq};
use crate::utilities::empty;
use hyper::{Response, StatusCode};

// router function; routes incoming requests to send back the appropriate response
pub async fn router(request: SvcReq, state: AppState) -> Result<RouterResponse, ApiError> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    tracing::info!(
        "📡 Incoming request: {} {}",
        request.method(),
        request.uri().path()
    );
    let routes = state.routes.clone();
    match routes.at(&method, &path) {
        Some((handler_fn, params)) => {
            tracing::info!("✅ Route matched with params: {:?}", params);
            handler_fn(request, state, params).await
        }
        None => {
            tracing::warn!("🚫 No route matched: {}", path);
            tracing::info!("Not found handler reached");
            let mut not_found_response = Response::new(empty());
            *not_found_response.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found_response)
        }
    }
}
