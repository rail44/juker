use hyper::Server;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower::make::Shared;

#[tokio::main]
async fn main() {
    // tokio::spawn(async {
    //     let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    //     let socket = TcpListener::bind(&addr).await.unwrap();
    //     while let Ok((stream, _)) = socket.accept().await {
    //         tokio_tungstenite::accept_async(stream).await.unwrap();
    //     }
    // });

    let svc = Shared::new(ServeDir::new("src"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr).serve(svc).await.unwrap();
}
