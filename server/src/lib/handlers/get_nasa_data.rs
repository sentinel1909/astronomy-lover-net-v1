// src/lib/handlers/get_nasa_data.rs

// dependencies
use crate::errors::ApiError;
use crate::service::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::Local;
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

// get endpoint handler to retrieve cached NASA API date
pub async fn from_cached(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let today = Local::now().naive_local();
    let formatted_date = today.format("%Y-%m-%d").to_string();
    println!("{}", today);
    let conn = state
        .db_client
        .connect()
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let mut result = conn
        .query(
            "SELECT * FROM nasa_api_data WHERE date = ?1",
            [formatted_date],
        )
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let response = result
        .next()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    if let Some(row) = response {
        let copyright: Option<String> = Some(row.get(1).unwrap_or_default());
        let date: String = row.get(2).unwrap_or_default();
        let explanation: String = row.get(3).unwrap_or_default();
        let hdurl = row.get(4).unwrap_or_default();
        let media_type: String = row.get(5).unwrap_or_default();
        let title: String = row.get(6).unwrap_or_default();
        let url: String = row.get(7).unwrap_or_default();

        let body = NasaData {
            copyright,
            date,
            explanation,
            hdurl,
            media_type,
            title,
            url,
        };

        let response_body = Json(body);

        Ok((StatusCode::OK, response_body).into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}
