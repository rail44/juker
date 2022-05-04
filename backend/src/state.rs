use axum::{extract::ws, extract::ws::Message};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct State {
    pointer: RwLock<Pointer>,
    pub begin: RwLock<i64>,
    pub queue: RwLock<Vec<VideoRequest>>,
    pub txs: RwLock<Vec<UnboundedSender<Message>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRequest {
    author: String,
    id: String,
    like: Option<String>,
}

impl VideoRequest {
    pub fn new(author: String, id: String, like: Option<String>) -> Self {
        VideoRequest { author, id, like }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Pointer {
    Playing(usize),
    Stopping,
}

impl Default for Pointer {
    fn default() -> Self {
        Self::Stopping
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            begin: RwLock::new(Utc::now().timestamp()),
            pointer: Default::default(),
            queue: Default::default(),
            txs: Default::default(),
        }
    }
}
impl State {
    pub async fn get_response(&self) -> StateResponse {
        StateResponse {
            pointer: self.pointer.read().await.clone(),
            queue: self.queue.read().await.clone(),
            duration: Utc::now().timestamp() - *self.begin.read().await,
            listeners: self.txs.read().await.len(),
        }
    }

    pub async fn broadcast(&self) {
        for sender in self.txs.read().await.clone() {
            sender
                .send(ws::Message::Text(
                    serde_json::to_string(&self.get_response().await).unwrap(),
                ))
                .ok();
        }
    }

    pub async fn feed(&self, next_pointer: usize) {
        let pointer = self.pointer.read().await.clone();
        match pointer {
            Pointer::Stopping => {}
            Pointer::Playing(pointer) => {
                if pointer == next_pointer {
                    return;
                }

                {
                    let queue = self.queue.read().await;
                    if queue.len() <= next_pointer {
                        self.assign_pointer(Pointer::Stopping).await;
                    } else {
                        self.play(next_pointer).await;
                    }
                }

                *self.begin.write().await = Utc::now().timestamp();
                self.broadcast().await;
            }
        }
    }

    pub async fn read_pointer(&self) -> Pointer {
        self.pointer.read().await.clone()
    }

    async fn assign_pointer(&self, p: Pointer) {
        *self.pointer.write().await = p;
    }

    pub async fn play(&self, i: usize) {
        self.assign_pointer(Pointer::Playing(i)).await;
        *self.begin.write().await = Utc::now().timestamp();
    }
}

#[derive(Serialize)]
pub struct StateResponse {
    pointer: Pointer,
    queue: Vec<VideoRequest>,
    duration: i64,
    listeners: usize,
}
