mod channel;
mod error;
mod network;
mod simulation;

use channel::Senders;
use error::Result;
use futures::future;
use futures::pin_mut;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let senders = Arc::new(Mutex::new(Senders::new()));
    let listener = TcpListener::bind(&addr).await?;
    let listen = network::listen(listener, senders);
    let run_sim = simulation::run();
    pin_mut!(listen, run_sim);
    future::select(listen, run_sim).await;

    Ok(())
}
