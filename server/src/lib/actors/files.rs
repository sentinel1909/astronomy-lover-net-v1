// src/lib/actors/files.rs

// dependencies
use std::path::PathBuf;
use tokio::spawn;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

// enum type to define possible messages for the files actor
pub enum FilesMessage {
    Get {
        path: String,
        reply: oneshot::Sender<(FilesResult, PathBuf)>,
    },
}

// enum type to represent errors more granularly
pub enum FilesResult {
    Ok(Vec<u8>),
    NotFound,
    IoError(std::io::Error),
}

// struct type to represent the files actor
pub struct FilesActor {
    base_path: PathBuf,
    rx: Receiver<FilesMessage>,
}

// methods for the files actor
impl FilesActor {
    pub fn start_files_actor() -> (Sender<FilesMessage>, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel::<FilesMessage>(32);
        let base_path = PathBuf::from("site");
        let files_actor = Self { base_path, rx };
        let files_handle = spawn(async move {
            files_actor.run().await;
        });
        tracing::info!("Files actor is go!");
        (tx, files_handle)
    }

    async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                FilesMessage::Get { mut path, reply } => {
                    tracing::info!("Message received: Path from request is: {:?}", path);
                    let needs_index = path.is_empty()
                        || path.ends_with('/')
                        || PathBuf::from(&path).extension().is_none();

                    if needs_index {
                        if path.is_empty() {
                            path = "index.html".to_string();
                        } else {
                            path = format!("{}/index.html", path.trim_end_matches('/'));
                        }
                    }

                    let full_path = self.base_path.join(path);
                    tracing::info!("Serving file: {:?}", full_path);
                    let result = match tokio::fs::read(full_path.clone()).await {
                        Ok(bytes) => FilesResult::Ok(bytes),
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => FilesResult::NotFound,
                        Err(e) => FilesResult::IoError(e),
                    };

                    let _ = reply.send((result, full_path));
                }
            }
        }
    }
}
