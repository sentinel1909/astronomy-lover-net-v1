// src/lib/handlers/get_nasa_data.rs

// dependencies
use crate::errors::ApiError;
use crate::service::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use domain::NasaData;
use url::Url;
use uuid::Uuid;

// get endpoint handler to retrieve data from the NASA API
pub async fn from_nasa_api(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let key = state.config.api_key;
    let api_key_segment = ["apod?api_key=", &key].concat();
    let nasa_api_url = Url::parse("https://api.nasa.gov/planetary/")
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .join(&api_key_segment)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let api_client = state.api_client;

    let response = api_client
        .get(nasa_api_url)
        .send()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .json::<NasaData>()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let conn = state
        .db_client
        .connect()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    conn.execute("INSERT INTO nasa_api_data (uid, copyright, date, explanation, hdurl, media_type, title, url) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", [Uuid::new_v4().to_string(), response.copyright.unwrap_or_default(), response.date, response.explanation, response.hdurl.unwrap_or_default(), response.media_type, response.title, response.url]).await.map_err(|e| {
        ApiError::Internal(e.to_string())
    })?;

    Ok(StatusCode::OK)
}
