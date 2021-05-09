//! Infrastructure for communication between tasks on the server via channels

use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::mpsc::UnboundedSender;

/// Contains the sender end of a channel that is consumed by the simulation task
/// and the sender end of the channels that are consumed by connection handler
/// tasks.
pub struct Senders {
    /// The sender end of a channel that is consumed by the simulation task.
    sim_sender: Option<UnboundedSender<MessageToSimulation>>,

    /// Contains the sender end of a channel for each connection handler task.
    conn_handler_senders: HashMap<SocketAddr, UnboundedSender<MessageToConnectionHandler>>,
}

impl Senders {
    pub fn new() -> Senders {
        Senders {
            sim_sender: None,
            conn_handler_senders: HashMap::new(),
        }
    }

    pub fn insert_sim_sender(&mut self, sender: UnboundedSender<MessageToSimulation>) {
        self.sim_sender = Some(sender);
    }

    pub fn insert_conn_handler_sender(
        &mut self,
        addr: SocketAddr,
        sender: UnboundedSender<MessageToConnectionHandler>,
    ) {
        self.conn_handler_senders.insert(addr, sender);
    }

    pub fn remove_conn_handler_sender(&mut self, addr: &SocketAddr) {
        self.conn_handler_senders.remove(&addr);
    }

    /// Attempts to send a message on the channel consumed by the simulation
    /// task.
    pub fn send_to_sim(&self, msg: MessageToSimulation) {
        if let Some(sender) = &self.sim_sender {
            let _ = sender.send(msg);
        }
    }

    /// Attempts to send a message on a channel consumed by a connection handler
    /// task.
    pub fn send_to_conn_handler(&self, addr: &SocketAddr, msg: MessageToConnectionHandler) {
        if let Some(sender) = &self.conn_handler_senders.get(addr) {
            let _ = sender.send(msg);
        }
    }
}

/// Message consumed by the simulation task.
#[derive(Debug)]
pub struct MessageToSimulation {}

/// Message consumed by a connection handler task.
#[derive(Debug)]
pub struct MessageToConnectionHandler {}
