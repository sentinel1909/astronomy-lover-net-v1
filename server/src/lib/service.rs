// src/lib/service.rs

// dependencies
use crate::actors::analytics::AnalyticsMessage;
use crate::actors::fetch::FetchMessage;
use crate::actors::files::FilesMessage;
use crate::actors::ping::PingMessage;
use crate::init::build_route_table;
use crate::middleware::Logger;
use crate::routes::router;
use crate::state::AppState;
use crate::utilities::shutdown_signal;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;
use std::pin::pin;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tower::ServiceBuilder;

// struct type to represent our service that runs on Shuttle
pub struct HyperService {
    pub analytics_tx: Sender<AnalyticsMessage>,
    pub files_tx: Sender<FilesMessage>,
    pub fetch_tx: Sender<FetchMessage>,
    pub ping_tx: Sender<PingMessage>,
}

// implement the Service trait for the HyperService type
#[shuttle_runtime::async_trait]
impl Service for HyperService {
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        // create the routing table and routes
        let table = build_route_table();

        // create the application state
        let state = AppState {
            analytics_tx: self.analytics_tx.clone(),
            fetch_tx: self.fetch_tx.clone(),
            files_tx: self.files_tx.clone(),
            ping_tx: self.ping_tx.clone(),
            routes: Arc::new(table),
        };

        // set up a listener, using the Shuttle provided address
        let listener = TcpListener::bind(addr).await?;

        // create a new http instance
        let http = http1::Builder::new();

        // create a new instance of GracefulShutdown
        let graceful = GracefulShutdown::new();

        // pin the shutdown_signal function in memory
        let mut signal = pin!(shutdown_signal());

        // the main loop, listen for incoming connections and serve the router to respond to
        // incoming requests
        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    let io = TokioIo::new(stream);
                    let state = state.clone();
                    let svc = service_fn(move |req| {
                        let state = state.clone();
                        async move { match router(req, state).await {
                            Ok(resp) => Ok::<_, hyper::Error>(resp),
                            Err(api_err) => {
                                tracing::error!("Request failed: {api_err}");
                                Ok(api_err.to_response())
                            }
                        }}
                    });
                    let svc = ServiceBuilder::new().layer_fn(Logger::new).service(svc);
                    let conn = http.serve_connection(io, svc );
                    let fut = graceful.watch(conn);
                    tokio::spawn(async move {
                        if let Err(e) = fut.await {
                            eprintln!("Error serving connection: {:?}", e);
                        }
                    });
                },

                _ = &mut signal => {
                    eprintln!("Graceful shutdown signal received...");
                    break Ok(());
                }
            }
        }
    }
}
