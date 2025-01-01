// src/main.rs

// dependencies
use astronomy_lover_net_v1_lib::configuration::AppConfig;
use astronomy_lover_net_v1_lib::service::{AppState, AstronomyLoverNetApplication};
use astronomy_lover_net_v1_lib::{get_subscriber, init_subscriber};
use reqwest::Client;
use shuttle_runtime::{Error, SecretStore, Secrets};

// main function
#[shuttle_runtime::main]
async fn main(#[Secrets] secrets: SecretStore) -> Result<AstronomyLoverNetApplication, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "astronomy-lover-net-v1".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // get the app configuration, including configuration and reqwest client
    let config = AppConfig::try_from(secrets)?;
    let client = Client::new();

    // set the application state
    let app_state = AppState { config, client };

    // build the app router
    let AstronomyLoverNetApplication(router) = AstronomyLoverNetApplication::build(app_state)?;

    Ok(AstronomyLoverNetApplication(router))
}
