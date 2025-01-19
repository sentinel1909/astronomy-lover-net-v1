// src/main.rs

// dependencies
use astronomy_lover_net_v1_lib::configuration::AppConfig;
use astronomy_lover_net_v1_lib::service::{AppState, AstronomyLoverNetApplication};
use astronomy_lover_net_v1_lib::{get_subscriber, init_subscriber};
use libsql::Database;
use reqwest::Client;
use shuttle_runtime::{Error, SecretStore, Secrets};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function
#[shuttle_runtime::main]
async fn main(
    #[Secrets] secrets: SecretStore,
    #[Turso(addr = "{secrets.TURSO_ADDR}", token = "{secrets.TURSO_TOKEN}")] client: Database,
) -> Result<AstronomyLoverNetApplication, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "astronomy-lover-net-v1".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let client = Arc::new(client);
    let conn = client.connect().unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS nasa_api_data (uid text primary key, copyright text, date text, explanation text, hdurl text, media_type text, title text, url text);", ())
        .await
        .unwrap();

    // get the app configuration, including configuration and reqwest client
    let config = AppConfig::try_from(secrets)?;
    let client = Client::new();

    // set the application state
    let app_state = AppState { config, client };

    // build the app router
    let AstronomyLoverNetApplication(router) = AstronomyLoverNetApplication::build(app_state)?;

    Ok(AstronomyLoverNetApplication(router))
}
