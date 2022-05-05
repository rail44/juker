use std::env;
use tokio::sync::OnceCell;

#[derive(Debug, PartialEq)]
pub enum Env {
    Dev,
    Prod,
}

static ENV: OnceCell<Env> = OnceCell::const_new();

pub async fn get_env() -> &'static Env {
    ENV.get_or_init(|| async {
        match env::var("ENV").as_ref().map(|s| s.as_str()) {
            Ok("dev") => Env::Dev,
            _ => Env::Prod,
        }
    })
    .await
}
