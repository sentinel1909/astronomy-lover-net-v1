// src/main.rs

// dependencies
use astronomy_lover_net_v1_lib::configuration::AppConfig;
use astronomy_lover_net_v1_lib::service::{AppState, AstronomyLoverNetApplication};
use astronomy_lover_net_v1_lib::{get_subscriber, init_subscriber};
use libsql::Database;
use reqwest::Client;
use shuttle_runtime::{CustomError, Error, SecretStore, Secrets};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function
#[shuttle_runtime::main]
async fn main(
    #[Secrets] secrets: SecretStore,
    #[Turso(addr = "{secrets.TURSO_DB_ADDR}", token = "{secrets.TURSO_DB_TOKEN}")] client: Database,
) -> Result<AstronomyLoverNetApplication, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "astronomy-lover-net-v1".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // create a database connection using the supplied Turso client
    let db_client = Arc::new(client);
    let conn = db_client.connect().map_err(|e| {
        let error_msg = format!("Unable to connect to the database: {}", e);
        CustomError::new(e).context(error_msg)
    })?;

    // create the database table
    conn.execute("CREATE TABLE IF NOT EXISTS nasa_api_data (uid TEXT PRIMARY KEY, copyright TEXT, date TEXT, explanation TEXT UNIQUE, hdurl TEXT, media_type TEXT, title TEXT, url TEXT);", ())
        .await
        .map_err(|e| {
            let error_msg = format!("Unable to create table in the database: {}", e);
            CustomError::new(e).context(error_msg)
        })?;

    // get the app configuration, including configuration. reqwest client, and Turso database client
    let config = AppConfig::try_from(secrets)?;
    let api_client = Client::new();

    // set the application state
    let app_state = AppState {
        config,
        api_client,
        db_client,
    };

    // build the app router
    let AstronomyLoverNetApplication(router) = AstronomyLoverNetApplication::build(app_state)?;

    Ok(AstronomyLoverNetApplication(router))
}
