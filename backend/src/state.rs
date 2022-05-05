use axum::{extract::ws, extract::ws::Message};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

// TODO: dump it for persistent
#[derive(Debug, Default)]
pub struct State {
    playing: RwLock<Option<PlayingStatus>>,
    pub queue: RwLock<Vec<VideoRequest>>,
    pub txs: RwLock<Vec<UnboundedSender<Message>>>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct PlayingStatus {
    pub pointer: usize,
    begin_milli: i64,
}

impl PlayingStatus {
    fn with_pointer(pointer: usize) -> Self {
        Self {
            pointer,
            begin_milli: Utc::now().timestamp_millis(),
        }
    }
}

impl State {
    pub async fn get_response(&self) -> StateResponse {
        StateResponse {
            playing: match self.read_playing().await {
                None => None,
                Some(PlayingStatus {
                    pointer,
                    begin_milli,
                }) => {
                    let req = self.get_video_request(pointer).await;
                    let duration_mill = Utc::now().timestamp_millis() - begin_milli;
                    Some(PlayingResponse {
                        id: req.id,
                        duration: (duration_mill as f32) / 1000_f32,
                        pointer,
                    })
                }
            },
            listeners: self.txs.read().await.len(),
            count: self.queue.read().await.len(),
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

    pub async fn get_video_request(&self, i: usize) -> VideoRequest {
        self.queue.read().await[i].clone()
    }

    pub async fn feed(&self, next_pointer: usize) {
        match self.read_playing().await {
            None => {}
            Some(PlayingStatus { pointer, .. }) => {
                if pointer == next_pointer {
                    return;
                }

                let queue = self.queue.read().await;
                if queue.len() <= next_pointer {
                    self.stop().await;
                } else {
                    self.play(next_pointer).await;
                }
            }
        }
    }

    pub async fn read_playing(&self) -> Option<PlayingStatus> {
        self.playing.read().await.clone()
    }

    async fn assign_playing(&self, p: Option<PlayingStatus>) {
        *self.playing.write().await = p;
    }

    pub async fn stop(&self) {
        self.assign_playing(None).await;
        self.broadcast().await;
        crate::slack::post_message("Playing completed").await;
    }

    pub async fn play(&self, pointer: usize) {
        self.assign_playing(Some(PlayingStatus::with_pointer(pointer)))
            .await;
        let req = self.get_video_request(pointer).await;
        self.broadcast().await;

        crate::slack::post_message(&format!(
            "Now playing {}, requested by {}",
            req.id, req.author
        ))
        .await;
        if let Some(like) = req.like {
            crate::slack::post_message(&format!("with recommended point \"{}\"", like)).await;
        }
        crate::slack::post_message("Join https://rail44.github.io/juker/ to watch").await;
    }
}

#[derive(Serialize)]
pub struct PlayingResponse {
    id: String,
    duration: f32,
    pointer: usize,
}

#[derive(Serialize)]
pub struct StateResponse {
    #[serde(flatten)]
    playing: Option<PlayingResponse>,

    listeners: usize,
    count: usize,
}
