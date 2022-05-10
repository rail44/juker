use anyhow::Result;
use axum::{
    extract::ws,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Extension, Form, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{
    sink::SinkExt,
    stream::{SplitStream, StreamExt},
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use url::Url;

mod env;
mod slack;
mod state;
mod youtube;

use state::{PlayingStatus, State, VideoRequest};

fn ws_chan(socket: WebSocket) -> (UnboundedSender<Message>, SplitStream<WebSocket>) {
    let (mut ws_sender, ws_receiver) = socket.split();
    let (tx_sender, tx_receiver) = unbounded_channel();
    tokio::spawn(async move {
        let mut stream = UnboundedReceiverStream::new(tx_receiver);
        while let Some(message) = stream.next().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
        stream.close();
    });

    (tx_sender, ws_receiver)
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum SocketMessage {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "feed")]
    Feed { position: usize },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("{:?}", env::get_env().await);
    slack::post_message("launched").await;

    let initial_state = State::default();

    // TODO: redirect to frontend when someone accessing to `/`
    let app = Router::new()
        .route("/state", get(state))
        .route("/status", get(status))
        .route("/command", post(command))
        .route("/interactive", post(interactive))
        .route("/request", post(request))
        .route("/socket", get(socket_upgrade))
        .layer(Extension(Arc::new(initial_state)));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn status() -> &'static str {
    "ok"
}

#[derive(Deserialize, Debug)]
struct CommandRequest {
    trigger_id: String,
    text: String,
}

async fn command(
    req: Form<CommandRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("{:?}", req);

    // TODO: help
    match req.text.as_str() {
        "next" => match state.read_playing().await {
            None => {
                if state.queue.read().await.is_empty() {
                    return (StatusCode::BAD_REQUEST, "queue is empty");
                }

                state.play(0).await;
                state.broadcast().await;
            }
            Some(PlayingStatus { position, .. }) => {
                let next_position = position + 1;
                let queue = state.queue.read().await;
                if queue.len() <= next_position {
                    state.stop().await;
                } else {
                    state.play(next_position).await;
                }
            }
        },
        "prev" => match state.read_playing().await {
            None => {
                let len = state.queue.read().await.len();
                if len == 0 {
                    return (StatusCode::BAD_REQUEST, "queue is empty");
                }

                state.play(len - 1).await;
                state.broadcast().await;
            }
            Some(PlayingStatus { position, .. }) => {
                let next_position = position.saturating_sub(1);
                let queue = state.queue.read().await;
                if queue.len() <= next_position {
                    state.stop().await;
                } else {
                    state.play(next_position).await;
                }
            }
        },
        "clear" => {
            state.queue.write().await.clear();
            state.stop().await;
        }
        "play" => {
            if state.read_playing().await != None {
                return (StatusCode::BAD_REQUEST, "already playing");
            }

            state.play(0).await;
            state.broadcast().await;
        }
        _ => {
            slack::view_open(&req.trigger_id).await;
        }
    }

    (StatusCode::OK, "nobody listening")
}

async fn interactive(
    req: Form<slack::InteractiveRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("{}", req.payload);

    let payload: slack::InteractivePayload = serde_json::from_str(&req.payload).unwrap();
    if let slack::InteractivePayload::Submission(payload) = payload {
        let url = Url::parse(&payload.view.state.values.url.input.value).unwrap();
        // TODO: get timestamp and use it for default duration
        let id = url
            .query_pairs()
            .find_map(|(k, v)| if k == "v" { Some(v) } else { None })
            .unwrap();

        if let Some(vid_req) = VideoRequest::new(
            payload.user.username,
            id.to_string(),
            payload.view.state.values.like.input.value,
        )
        .await
        {
            state.queue.write().await.push(vid_req);

            if state.read_playing().await == None {
                state.play(state.queue.read().await.len() - 1).await;
            }
            state.broadcast().await;
        }
    }

    StatusCode::OK
}

#[derive(Deserialize, Debug)]
struct RequestRequest {
    author: String,
    id: String,
    like: Option<String>,
}

async fn request(
    Json(req): Json<RequestRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if let Some(vid_req) = VideoRequest::new(req.author, req.id, req.like).await {
        state.queue.write().await.push(vid_req);

        if state.read_playing().await == None {
            state.play(state.queue.read().await.len() - 1).await;
        }
        state.broadcast().await;
    }
    StatusCode::OK
}

async fn state(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.get_response().await))
}

async fn socket_upgrade(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| socket_handler(socket, state))
}

async fn receive_message(
    msg: Message,
    state: &Arc<State>,
    sender: &UnboundedSender<Message>,
) -> Result<()> {
    let body = msg.into_text()?;
    tracing::info!("{}", body);

    if let Ok(msg) = serde_json::from_str(&body) {
        match msg {
            SocketMessage::Ping => sender.send(ws::Message::Text(serde_json::to_string(
                &state.get_response().await,
            )?))?,
            SocketMessage::Feed {
                position: next_position,
            } => state.feed(next_position).await,
        }
    };
    Ok(())
}

async fn socket_handler(socket: WebSocket, state: Arc<State>) {
    let (sender, mut receiver) = ws_chan(socket);
    // TODO: hashmap with unique id
    state.txs.write().await.push(sender.clone());
    state.broadcast().await;

    while let Some(Ok(msg)) = receiver.next().await {
        if let Err(e) = receive_message(msg, &state, &sender).await {
            tracing::error!("{}", e);
            break;
        }
    }

    state.txs.write().await.retain(|v| !sender.same_channel(v));
    state.broadcast().await;
}
