// src/lib/service.rs

// dependencies
use crate::configuration::AppConfig;
use crate::handlers::{get_nasa_data::from_nasa_api, health_check};
use crate::telemetry::MakeRequestUuid;
use anyhow::Result;
use axum::{http::HeaderName, routing::get, Router};
use libsql::Database;
use reqwest::Client;
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::layer::Layer;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the server application
pub struct AstronomyLoverNetApplication(pub Router);

// struct type to represent application state
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub api_client: Client,
    pub db_client: Arc<Database>,
}

// methods for the AstronomyLoverNetService type
impl AstronomyLoverNetApplication {
    // function to build and return a configured application that serves out the Zola built public assets
    pub fn build(state: AppState) -> Result<Self> {
        // create a layer for Cross Origin Resource sharing
        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));

        // create public assets, wrap them in a trace layer
        let public_assets = ServiceBuilder::new()
            .layer(&trace_layer)
            .service(ServeDir::new("public"));

        // build the router and wrap it with the telemetry layers
        let x_request_id = HeaderName::from_static("x-request-id");
        let api_routes = Router::new()
            .route("/health_check", get(health_check))
            .route("/nasa_data", get(from_nasa_api))
            .layer(cors)
            .with_state(state)
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            );

        // wrap the API routes with a layer to normalize incoming paths
        let api_router = NormalizePathLayer::trim_trailing_slash().layer(api_routes);

        // combine the api and public assets to make the app
        let app = Router::new()
            .nest_service("/api", api_router)
            .fallback_service(public_assets);

        Ok(Self(app))
    }

    // utility function to facilitate testing
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0).await.unwrap();
    }
}

// implement the Shuttle Service trait on the NasaImageryViewerService type
#[shuttle_runtime::async_trait]
impl Service for AstronomyLoverNetApplication {
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        let app_router = self.0;
        axum::serve(TcpListener::bind(addr).await?, app_router).await?;

        Ok(())
    }
}
