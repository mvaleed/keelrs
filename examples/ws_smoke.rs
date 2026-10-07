//! Live smoke test: run the server and connect to it manually, e.g.
//!
//! ```sh
//! cargo run --example ws_smoke
//! websocat ws://127.0.0.1:9000
//! > "ping"
//! < "pong"
//! ```

use log::info;
use std::time::Duration;

use keelrs::{Result, Server};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting worker");

    Server::new("127.0.0.1:9000".parse().unwrap())
        .idle_timeout(Duration::from_secs(30))
        .run()
        .await
}
