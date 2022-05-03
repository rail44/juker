use axum::{
    extract::{Extension, Form, Json},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Serialize, Deserialize, Serializer};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber;
use url::Url;

mod slack;

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

#[derive(Default, Debug)]
struct State {
    pointer: usize,
    queue: RwLock<Vec<VideoRequest>>,
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        json!({
            "pointer": self.pointer,
            "queue": self.queue.read().unwrap().clone(),
        }).serialize(serializer)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let initial_state = State::default();

    initial_state.queue.write().unwrap().push(VideoRequest::new("rail44".into(), "AlXGFHExSL4".into(), Some("ここすき".into())));

    let app = Router::new()
        .route("/state", get(state))
        .route("/status", get(status))
        .route("/command", post(command))
        .route("/interactive", post(interactive))
        .route("/request", post(request))
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
    state.queue.write().unwrap().push(vid_req.clone());
    tracing::info!("{:?}", vid_req);
    StatusCode::OK
}

async fn request(
    Json(req): Json<VideoRequest>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("{:?}", req);
    state.queue.write().unwrap().push(req);
    StatusCode::OK
}

async fn state(
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(state))
}
