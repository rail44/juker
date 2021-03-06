use crate::youtube;
use crate::youtube::VideoProps;
use anyhow::{bail, Result};
use axum::{extract::ws, extract::ws::Message};
use chrono::Utc;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
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
    like: Option<String>,
    video: VideoProps,
}

impl VideoRequest {
    pub async fn new(author: String, id: String, like: Option<String>) -> Result<Self> {
        let video = youtube::get_video_props(&id).await?;
        Ok(VideoRequest {
            author,
            video,
            like,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayingStatus {
    pub position: usize,
    begin_milli: i64,
}

impl PlayingStatus {
    fn with_position(position: usize) -> Self {
        Self {
            position,
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
                    position,
                    begin_milli,
                }) => {
                    let req = self.get_video_request(position).await;
                    let duration_mill = Utc::now().timestamp_millis() - begin_milli;
                    Some(PlayingResponse {
                        id: req.video.id,
                        duration: (duration_mill as f32) / 1000_f32,
                        position,
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

    pub async fn feed(&self, next_position: usize) {
        match self.read_playing().await {
            None => {}
            Some(PlayingStatus { position, .. }) => {
                if position == next_position {
                    return;
                }

                let queue = self.queue.read().await;
                if queue.len() <= next_position {
                    self.stop().await;
                } else {
                    self.play(next_position).await;
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
    }

    pub async fn play(&self, position: usize) {
        self.assign_playing(Some(PlayingStatus::with_position(position)))
            .await;
        let req = self.get_video_request(position).await;
        self.broadcast().await;

        crate::slack::post_block_message(crate::slack::req_info_payload(
            &req.video,
            &req.author,
            req.like.as_deref(),
        ))
        .await;
    }

    pub async fn shuffle(&self) -> Result<()> {
        let mut queue = self.queue.write().await;
        if queue.len() == 0 {
            bail!("queue is empty");
        }

        let at = match self.read_playing().await {
            None => 0,
            Some(PlayingStatus { position, .. }) => position,
        };

        let mut target = queue.split_off(at);
        target.shuffle(&mut SmallRng::from_entropy());
        queue.append(&mut target);
        crate::slack::post_message("shuffled remains of queue").await;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct PlayingResponse {
    id: String,
    duration: f32,
    position: usize,
}

#[derive(Serialize)]
pub struct StateResponse {
    playing: Option<PlayingResponse>,
    listeners: usize,
    count: usize,
}
