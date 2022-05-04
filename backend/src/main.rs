use axum::{
    extract::ws,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Extension, Form, Json},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use futures::{
    sink::SinkExt,
    stream::{SplitStream, StreamExt},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::RwLock;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tower_http::trace::TraceLayer;
use tracing::Span;
use url::Url;

mod slack;

fn ws_chan(socket: WebSocket) -> (UnboundedSender<Message>, SplitStream<WebSocket>) {
    let (mut ws_sender, ws_receiver) = socket.split();
    let (tx_sender, tx_receiver) = unbounded_channel();
    tokio::spawn(async move {
        let mut stream = UnboundedReceiverStream::new(tx_receiver);
        while let Some(message) = stream.next().await {
            if ws_sender.send(message).await.is_err() {
                stream.close();
            }
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VideoRequest {
    author: String,
    id: String,
    like: Option<String>,
}

impl VideoRequest {
    fn new(author: String, id: String, like: Option<String>) -> Self {
        VideoRequest { author, id, like }
    }
}

#[derive(Debug)]
struct State {
    pointer: RwLock<usize>,
    begin: RwLock<i64>,
    queue: RwLock<Vec<VideoRequest>>,
    txs: RwLock<Vec<UnboundedSender<Message>>>,
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

#[derive(Serialize)]
struct StateResponse {
    pointer: usize,
    queue: Vec<VideoRequest>,
    duration: i64,
}

impl State {
    async fn get_response(&self) -> StateResponse {
        StateResponse {
            pointer: *self.pointer.read().await,
            queue: self.queue.read().await.clone(),
            duration: Utc::now().timestamp() - *self.begin.read().await,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let initial_state = State::default();

    initial_state.queue.write().await.push(VideoRequest::new(
        "rail44".into(),
        "BDrdOKcqlYc".into(),
        Some("ここすき".into()),
    ));
    initial_state.queue.write().await.push(VideoRequest::new(
        "rail44".into(),
        "oPELPOgDyXA".into(),
        Some("ここすき".into()),
    ));

    let app = Router::new()
        .route("/state", get(state))
        .route("/status", get(status))
        .route("/command", post(command))
        .route("/interactive", post(interactive))
        .route("/request", post(request))
        .route("/socket", get(socket_upgrade))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|req: &Request<_>, _: &Span| tracing::info!("{:?}", req))
                .on_response(|res: &Response<_>, _: Duration, _: &Span| {
                    tracing::info!("{:?}", res)
                }),
        )
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

#[derive(Deserialize)]
struct CommandRequest {
    trigger_id: String,
}

async fn command(req: Form<CommandRequest>) -> impl IntoResponse {
    slack::view_open(&req.trigger_id).await;

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
    tracing::info!("{:?}", vid_req);
    StatusCode::OK
}

async fn request(
    Json(req): Json<VideoRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("{:?}", req);
    state.queue.write().await.push(req);
    StatusCode::OK
}

async fn state(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    tracing::info!("{:?}", *state.txs.read().await);
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
    state.txs.write().await.push(sender.clone());

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
                } => {
                    if *state.pointer.read().await != next_pointer {
                        {
                            let queue = state.queue.read().await;
                            if queue.len() <= next_pointer {
                                *state.pointer.write().await = 0;
                            } else {
                                *state.pointer.write().await = next_pointer;
                            }
                        }

                        *state.begin.write().await = Utc::now().timestamp();

                        tracing::info!("{:?}", *state.txs.read().await);
                        for sender in state.txs.read().await.clone() {
                            if sender.is_closed() {
                                state.txs.write().await.retain(|v| !sender.same_channel(v));
                                continue;
                            }
                            if let Err(_) = sender
                                .send(ws::Message::Text(
                                    serde_json::to_string(&state.get_response().await).unwrap(),
                                )) {
                                    state.txs.write().await.retain(|v| !sender.same_channel(v));
                                }
                        }
                    }
                }
            }
        };
    }
}
