// src/lib/handlers/get_nasa_data.rs

// dependencies
use crate::errors::ApiError;
use crate::service::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use domain::NasaData;
use serde_json::json;
use url::Url;

// get endpoint handler to retrieve data from the NASA API
pub async fn from_nasa_api(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let key = state.config.api_key;
    let api_key_segment = ["apod?api_key=", &key].concat();
    let nasa_api_url = Url::parse("https://api.nasa.gov/planetary/")
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .join(&api_key_segment)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let api_client = state.client;

    let response = api_client
        .get(nasa_api_url)
        .send()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .json::<NasaData>()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let response_body = json!(response);

    Ok((StatusCode::OK, Json(response_body)).into_response())
}
