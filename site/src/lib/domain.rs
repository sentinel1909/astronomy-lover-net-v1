// src/lib/domain.rs

// dependencies
use serde::Deserialize;

// struct to represent the data returned from the NASA APOD API
#[derive(Deserialize, Debug, Clone)]
pub struct NASAData {
    pub date: String,
    pub title: String,
    pub explanation: String,
    pub copyright: Option<String>,
    pub media_type: String,
    pub url: String,
    pub hdurl: Option<String>,
}

// implement the default trait for the NASAData struct
impl Default for NASAData {
    fn default() -> Self {
        Self {
            date: "".to_string(),
            title: "".to_string(),
            explanation: "".to_string(),
            copyright: Some("".to_string()),
            media_type: "".to_string(),
            url: "".to_string(),
            hdurl: Some("".to_string()),
        }
    }
}
