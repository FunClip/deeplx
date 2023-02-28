mod deeplx;
mod server;
use crate::server::start_server;

#[tokio::main]
async fn main() {
    start_server("0.0.0.0:14869").await;
}