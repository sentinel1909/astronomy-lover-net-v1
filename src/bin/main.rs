// src/main.rs

// dependencies
use astronomy_lover_net_lib::HyperService;
use astronomy_lover_net_lib::actors::{AnalyticsActor, FetchActor, FilesActor, PingCounterActor};
use astronomy_lover_net_lib::types::HyperServiceError;
use shuttle_runtime::SecretStore;

// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<HyperService, HyperServiceError> {
    // get the NASA APOD API key from secrets
    let api_key = secrets.get("NASA_API_KEY").expect("API key was not found.");

    // start up the actors
    let (analytics_tx, _analytics_handle) = AnalyticsActor::start_analytics_actor();
    let (fetch_tx, _fetch_handle) = FetchActor::start_fetch_actor(api_key);
    let (files_tx, _files_handle) = FilesActor::start_files_actor();
    let (ping_tx, _ping_handle) = PingCounterActor::start_ping_actor();

    // start up the service
    Ok(HyperService {
        analytics_tx,
        fetch_tx,
        files_tx,
        ping_tx,
    })
}
