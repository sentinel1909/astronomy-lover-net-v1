// src/lib/actors/analytics.rs

// dependencies
use std::collections::HashMap;
use tokio::spawn;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

// enum type to define the possible messages for the analytics actor
pub enum AnalyticsMessage {
    GetCount {
        key: String,
        reply: oneshot::Sender<usize>,
    },
    Increment {
        key: String,
        reply: Option<oneshot::Sender<()>>,
    },
    GetAll {
        reply: oneshot::Sender<HashMap<String, usize>>,
    },
    Reset,
}

// struct type to represent the Analytics actor
pub struct AnalyticsActor {
    rx: Receiver<AnalyticsMessage>,
}

// methods for the AnalyticsActor type
impl AnalyticsActor {
    pub fn start_analytics_actor() -> (Sender<AnalyticsMessage>, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel::<AnalyticsMessage>(32);
        let analytics_actor = Self { rx };
        let analytics_handle = spawn(async move {
            analytics_actor.run().await;
        });
        tracing::info!("Analytics actor is go!");
        (tx, analytics_handle)
    }

    async fn run(mut self) {
        let mut counters: HashMap<String, usize> = HashMap::new();
        while let Some(msg) = self.rx.recv().await {
            match msg {
                AnalyticsMessage::GetCount { reply, key } => {
                    let count = counters.get(&key).copied().unwrap_or(0);
                    let _ = reply.send(count);
                }
                AnalyticsMessage::Increment { reply, key } => {
                    let count = counters.entry(key).or_insert(0);
                    *count += 1;
                    if let Some(reply) = reply {
                        let _ = reply.send(());
                    }
                }
                AnalyticsMessage::GetAll { reply } => {
                    let _ = reply.send(counters.clone());
                }
                AnalyticsMessage::Reset => {
                    counters.clear();
                    tracing::info!("Analytics stats reset.");
                }
            }
        }

        tracing::info!("AnalyticsActor shutting down.");
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn returns_zero_for_unknown_event() {
        let (tx, rx) = oneshot::channel::<usize>();
        let msg = AnalyticsMessage::GetCount {
            key: "ping".to_string(),
            reply: tx,
        };
        let (actor_tx, _handle) = AnalyticsActor::start_analytics_actor();
        actor_tx.send(msg).await.expect("failed to send message");
        let count = rx.await.expect("actor did not respond");
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn returns_one_after_one_increment() {
        let (actor_tx, _handle) = AnalyticsActor::start_analytics_actor();
        let (inc_tx, inc_rx) = oneshot::channel();
        let inc_msg = AnalyticsMessage::Increment {
            key: "ping".to_string(),
            reply: Some(inc_tx),
        };

        actor_tx
            .send(inc_msg)
            .await
            .expect("failed to send increment");
        inc_rx.await.expect("actor did not ack increment");

        let (get_tx, get_rx) = oneshot::channel();
        let get_msg = AnalyticsMessage::GetCount {
            key: "ping".to_string(),
            reply: get_tx,
        };

        actor_tx.send(get_msg).await.expect("failed to send get");
        let count = get_rx.await.expect("actor did not reply");

        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn returns_all_counts() {
        let (actor_tx, _handle) = AnalyticsActor::start_analytics_actor();

        for key in ["ping", "ping", "pong"] {
            let (tx, rx) = oneshot::channel();
            let msg = AnalyticsMessage::Increment {
                key: key.to_string(),
                reply: Some(tx),
            };
            actor_tx.send(msg).await.expect("send failed");
            rx.await.expect("actor did not ack");
        }

        let (tx, rx) = oneshot::channel();
        let msg = AnalyticsMessage::GetAll { reply: tx };
        actor_tx.send(msg).await.expect("send failed");

        let counts = rx.await.expect("actor did not respond");

        let mut expected = HashMap::new();
        expected.insert("ping".to_string(), 2);
        expected.insert("pong".to_string(), 1);

        assert_eq!(counts, expected);
    }
}
