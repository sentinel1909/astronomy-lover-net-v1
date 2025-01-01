// src/lib/handlers/get_nasa_data.rs

// dependencies
use crate::service::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

// struct to represent the data returned from the NASA APOD API
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct NasaData {
    pub date: String,
    pub title: String,
    pub explanation: String,
    pub copyright: Option<String>,
    pub media_type: String,
    pub url: String,
    pub hdurl: Option<String>,
}

// get endpoint handler to retrieve data from the NASA API
pub async fn from_nasa_api(State(state): State<AppState>) -> impl IntoResponse {
    let key = state.config.api_key;
    let api_key = ["apod?api_key=", &key].concat();
    let api_url = Url::parse("https://api.nasa.gov/planetary/")
        .unwrap()
        .join(&api_key)
        .unwrap();
    let api_client = state.client;

    let response = api_client
        .get(api_url)
        .send()
        .await
        .unwrap()
        .json::<NasaData>()
        .await
        .unwrap();

    let response_body = json!(response);

    (StatusCode::OK, Json(response_body)).into_response()
}