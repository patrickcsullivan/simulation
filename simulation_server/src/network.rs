//! Infrastructure for communication between the server and the network

use crate::channel::{MessageToSimulation, Senders};
use crate::error::Result;
use futures::future;
use futures::stream::{StreamExt, TryStreamExt};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel;

/// Handles a TCP connection initiated by a client.
async fn handle_connection(
    socket: TcpStream,
    addr: SocketAddr,
    senders: Arc<Mutex<Senders>>,
) -> Result<()> {
    let web_socket = tokio_tungstenite::accept_async(socket).await?;
    let (_outgoing, incoming) = web_socket.split();

    // Create a MPSC channel that the connection handler task will consume, and
    // save the sender end of the channel into a data structure that is shared
    // across tasks.
    let (sender, _receiver) = unbounded_channel();
    senders
        .lock()
        .unwrap()
        .insert_conn_handler_sender(addr, sender);

    // Handle each incoming WS message by sending a message to the simulation
    // task.
    let handle_connection_messages = incoming.try_for_each(|msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        senders.lock().unwrap().send_to_sim(MessageToSimulation {});
        future::ok(())
    });

    // let handle_channel_messages = receiver.map(|m| -> {""});

    let _ = handle_connection_messages.await;
    senders.lock().unwrap().remove_conn_handler_sender(&addr);
    Ok(())
}

/// Accept and handle each new TCP connection.
pub async fn listen(listener: TcpListener, senders: Arc<Mutex<Senders>>) -> Result<()> {
    while let Ok((socket, addr)) = listener.accept().await {
        let senders = senders.clone();
        tokio::spawn(async move { handle_connection(socket, addr, senders).await });
    }
    Ok(())
}
