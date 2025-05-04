// src/lib/actors/fetch.rs

// dependencies
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use tokio::spawn;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use url::Url;

// enum type to define the possible messages for the Fetch actor
pub enum FetchMessage {
    Get { reply: oneshot::Sender<FetchResult> },
}

// enum type to represent fetch errors more granularly
pub enum FetchResult {
    Ok(NasaData),
    FetchError(ReqwestError),
}

// struct type to represent the Fetch actor
pub struct FetchActor {
    nasa_api_key: String,
    client: Client,
    rx: Receiver<FetchMessage>,
}

// struct to represent the data returned from the NASA APOD API
#[derive(Debug, Deserialize, Serialize)]
pub struct NasaData {
    pub date: String,
    pub title: String,
    pub explanation: String,
    pub copyright: Option<String>,
    pub media_type: String,
    pub url: String,
    pub hdurl: Option<String>,
}

// methods for the FetchActor type
impl FetchActor {
    pub fn start_fetch_actor(api_key: String) -> (Sender<FetchMessage>, JoinHandle<()>) {
        let client = Client::new();
        let nasa_api_key = api_key;
        let (tx, rx) = mpsc::channel::<FetchMessage>(32);
        let fetch_actor = Self { client, rx, nasa_api_key };
        let fetch_handle = spawn(async move {
            fetch_actor.run().await;
        });
        tracing::info!("Fetch actor is go!");
        (tx, fetch_handle)
    }

    async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                FetchMessage::Get { reply } => {
                    let url = build_api_url(self.nasa_api_key.clone());
                    let result = match self.client.get(url).send().await {
                        Ok(response) => match response.json::<NasaData>().await {
                            Ok(data) => FetchResult::Ok(data),
                            Err(e) => FetchResult::FetchError(e),
                        },
                        Err(e) => FetchResult::FetchError(e),
                    };

                    let _ = reply.send(result);
                }
            }
        }

        tracing::info!("Fetch actor shutting down.");
    }
}

// function to build the API url, including the API key
fn build_api_url(key: String) -> Url {
    let api_key = ["apod?api_key=", &key].concat();
    let api_url = Url::parse("https://api.nasa.gov/planetary/")
        .expect("Failed to parse the url to fetch data from.");
    api_url
        .join(&api_key)
        .expect("Failed to join URL with the API key.")
}
