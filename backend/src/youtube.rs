use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use tokio::sync::OnceCell;

static YOUTUBE_TOKEN: OnceCell<String> = OnceCell::const_new();
async fn get_token() -> &'static str {
    YOUTUBE_TOKEN
        .get_or_init(|| async { env::var("YOUTUBE_TOKEN").unwrap_or_else(|_| "".to_string()) })
        .await
}

#[derive(Debug, Deserialize)]
struct VideoListResponse {
    items: Vec<VideoResponse>,
}

#[derive(Debug, Deserialize)]
struct VideoResponse {
    id: String,
    snippet: SnippetResponse,
}

#[derive(Debug, Deserialize)]
struct SnippetResponse {
    title: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VideoProps {
    pub id: String,
    pub title: String,
    pub channel: String,
}

impl From<VideoListResponse> for Result<VideoProps> {
    fn from(res: VideoListResponse) -> Self {
        let video = &res.items.get(0).with_context(|| "video not found")?;
        let snippet = &video.snippet;
        Ok(VideoProps {
            id: video.id.clone(),
            title: snippet.title.clone(),
            channel: snippet.channel_title.clone(),
        })
    }
}

pub async fn get_video_props(id: &str) -> Result<VideoProps> {
    let token = get_token().await;
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.googleapis.com/youtube/v3/videos")
        .query(&[("key", token), ("part", "snippet"), ("id", id)])
        .send()
        .await?;
    let res: VideoListResponse = res.json().await?;
    tracing::info!("{:?}", res);
    res.into()
}
