// src/main.rs

// dependencies
use astronomy_lover_net_lib::FilesActor;
use astronomy_lover_net_lib::HyperService;
use astronomy_lover_net_lib::actors::AnalyticsActor;
use astronomy_lover_net_lib::actors::PingCounterActor;
use astronomy_lover_net_lib::types::HyperServiceError;


// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn main() -> Result<HyperService, HyperServiceError> {
    // start up the actors
    let (analytics_tx, _analytics_handle) = AnalyticsActor::start_analytics_actor();
    let (files_tx, _files_handle) = FilesActor::start_files_actor();
    let (ping_tx, _ping_handle) = PingCounterActor::start_ping_actor();

    // start up the service
    Ok(HyperService {
        analytics_tx,
        files_tx,
        ping_tx,
    })
}