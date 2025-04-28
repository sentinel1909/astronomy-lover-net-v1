// src/lib/utilities.rs

// dependencies
use crate::types::JsonResponse;
use http_body_util::{
    Empty, Full,
    {BodyExt, combinators::BoxBody},
};
use hyper::body::Bytes;
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::{Error, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;
use tokio::signal;

// static variable to represent the mime-map
static MIME_MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

// Helper function to initialize the map
fn create_mime_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("html", "text/html"),
        ("css", "text/css"),
        ("js", "application/javascript"),
        ("ico", "image/x-icon"),
        ("json", "application/json"),
        ("xml", "application/atom+xml"),
        ("png", "image/png"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("gif", "image/gif"),
        ("svg", "image/svg+xml"),
        ("txt", "text/plain"),
        ("pdf", "application/pdf"),
        ("wasm", "application/wasm"),
    ])
}

// utility function which provides a shutdown signal; leverage Tokio::signal
pub async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");
}

// utility function to create an empty body for a Response
pub fn empty() -> BoxBody<Bytes, Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

// utility functo for setting the content type header to "application/json"
pub fn set_content_type_json<T>(response: &mut Response<T>) {
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
}

// utility function to create a JSON response body
pub fn json_response_msg<T: Serialize>(value: T) -> BoxBody<Bytes, Error> {
    let wrapper = JsonResponse {
        msg: "success",
        content: value,
    };

    let json = serde_json::to_vec(&wrapper).unwrap_or_else(|e| {
        let fallback = format!(r#"{{"msg":"error","error":"{}"}}"#, e);
        fallback.into_bytes()
    });

    Full::new(Bytes::from(json))
        .map_err(|never| match never {})
        .boxed()
}
// utility function to build a response message from arbitrary bytes
pub fn bytes_response_msg<T: Into<Bytes>>(body: T, path: &str) -> Response<BoxBody<Bytes, Error>> {
    let original_path = if path.is_empty() { "index.html" } else { path };
    let mime = get_mime_type(original_path);
    tracing::info!("📝 Guessed MIME type for {}: {}", path, mime);

    let full = Full::new(body.into())
        .map_err(|never| match never {})
        .boxed();
    let mut response = Response::new(full);

    if let Ok(content_type) = HeaderValue::from_str(mime) {
        response.headers_mut().insert(CONTENT_TYPE, content_type);
    } else {
        tracing::warn!("⚠️ Failed to set Content-Type header for MIME: {}", mime);
    }

    response
}

// utility function to get the mime type
pub fn get_mime_type(filename: &str) -> &str {
    let map = MIME_MAP.get_or_init(create_mime_map);
    const DEFAULT_MIME_TYPE: &str = "application/octet-stream";

    Path::new(filename)
        .extension()
        .and_then(|os_str| os_str.to_str())
        .and_then(|ext| map.get(ext))
        .copied()
        .unwrap_or(DEFAULT_MIME_TYPE)
}

// utility to parse query parameters into individual pieces
// Parse "key=value&foo=bar" into HashMap<String, String>
pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_returns_empty_map() {
        let result = parse_query_string("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_html_extension() {
        assert_eq!(
            get_mime_type("index.html"),
            "text/html",
            "Test failed for .html"
        );
    }

    #[test]
    fn test_css_extension() {
        assert_eq!(
            get_mime_type("style.css"),
            "text/css",
            "Test failed for .css"
        );
    }

    #[test]
    fn test_js_extension() {
        assert_eq!(
            get_mime_type("script.js"),
            "application/javascript",
            "Test failed for .js"
        );
    }

    #[test]
    fn test_ico_extension() {
        assert_eq!(
            get_mime_type("favicon.ico"),
            "image/x-icon",
            "Test failed for .ico"
        );
    }

    #[test]
    fn test_json_extension() {
        assert_eq!(
            get_mime_type("data.json"),
            "application/json",
            "Test failed for .json"
        );
    }

    #[test]
    fn test_xml_extension() {
        assert_eq!(
            get_mime_type("feed.xml"),
            "application/atom+xml",
            "Test failed for .xml"
        );
    }

    #[test]
    fn test_unknown_extension() {
        assert_eq!(
            get_mime_type("archive.zip"),
            "application/octet-stream",
            "Test failed for unknown extension"
        );
    }

    #[test]
    fn test_no_extension() {
        assert_eq!(
            get_mime_type("README"),
            "application/octet-stream",
            "Test failed for filename with no extension"
        );
    }

    #[test]
    fn test_empty_filename() {
        // The Path::new("").extension() returns None, so this should be default
        assert_eq!(
            get_mime_type(""),
            "application/octet-stream",
            "Test failed for empty filename"
        );
    }
}
