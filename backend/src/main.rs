use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::trace::{
    DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;
use tracing_subscriber;
use url::Url;

mod slack;

#[derive(Debug, Clone)]
struct VideoRequest {
    author: String,
    id: String,
    like: String,
}

impl VideoRequest {
    fn new(author: String, id: String, like: String) -> Self {
        VideoRequest { author, id, like }
    }
}

struct State {
    queue: Arc<RwLock<Vec<VideoRequest>>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(State::default());

    let app = Router::new()
        .route("/status", get(status))
        .route("/command", post(command))
        .route("/command", get(command))
        .route("/interactive", post(interactive))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::INFO))
                .on_eos(DefaultOnEos::new().level(Level::INFO)),
        )
        .layer(Extension(state));
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
    let url = Url::parse(&req.payload.state.values.url.text.value).unwrap();
    let id = url
        .query_pairs()
        .find_map(|(k, v)| if k == "v" { Some(v) } else { None })
        .unwrap();

    let vid_req = VideoRequest::new(
        req.payload.user.username.to_string(),
        id.to_string(),
        req.payload.state.values.like.text.value.to_string(),
    );
    state.queue.write().unwrap().push(vid_req.clone());
    tracing::info!("{:?}", vid_req);
    StatusCode::OK
}
