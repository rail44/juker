use axum::{
    extract::ws,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Extension, Form, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
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

mod slack;
mod state;

use state::{Pointer, State, VideoRequest};

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
    Feed { pointer: usize },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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

    // TODO: help, next, prev
    match req.text.as_str() {
        "play" => {
            if *state.pointer.read().await != Pointer::Stopping {
                return StatusCode::OK;
            }

            if state.txs.read().await.len() == 0 {
                return StatusCode::OK;
            }

            *state.pointer.write().await = Pointer::Playing(0);
            *state.begin.write().await = Utc::now().timestamp();
            state.broadcast().await;
        }
        _ => {
            slack::view_open(&req.trigger_id).await;
        }
    }

    StatusCode::OK
}

async fn interactive(
    req: Form<slack::InteractiveRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("{}", req.payload);

    let payload: slack::InteractivePayload = serde_json::from_str(&req.payload).unwrap();
    let url = Url::parse(&payload.view.state.values.url.input.value).unwrap();
    let id = url
        .query_pairs()
        .find_map(|(k, v)| if k == "v" { Some(v) } else { None })
        .unwrap();

    let vid_req = VideoRequest::new(
        payload.user.username,
        id.to_string(),
        payload.view.state.values.like.input.value,
    );
    state.queue.write().await.push(vid_req.clone());

    if *state.pointer.read().await == Pointer::Stopping {
        *state.pointer.write().await = Pointer::Playing(state.queue.read().await.len() - 1);
        *state.begin.write().await = Utc::now().timestamp();
        state.broadcast().await;
    }
    StatusCode::OK
}

async fn request(
    Json(req): Json<VideoRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("{:?}", req);
    state.queue.write().await.push(req);

    if *state.pointer.read().await == Pointer::Stopping {
        *state.pointer.write().await = Pointer::Playing(state.queue.read().await.len() - 1);
        *state.begin.write().await = Utc::now().timestamp();
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

async fn socket_handler(socket: WebSocket, state: Arc<State>) {
    let (sender, mut receiver) = ws_chan(socket);
    // TODO: hashmap with unique id
    state.txs.write().await.push(sender.clone());
    state.broadcast().await;

    while let Some(msg) = receiver.next().await {
        let body = msg.unwrap().into_text().unwrap();
        tracing::info!("{}", body);

        if let Ok(msg) = serde_json::from_str(&body) {
            match msg {
                SocketMessage::Ping => {
                    sender
                        .send(ws::Message::Text(
                            serde_json::to_string(&state.get_response().await).unwrap(),
                        ))
                        .unwrap();
                }
                SocketMessage::Feed {
                    pointer: next_pointer,
                } => state.feed(next_pointer).await
            }
        };
    }

    state.txs.write().await.retain(|v| !sender.same_channel(v));
    state.broadcast().await;
}
