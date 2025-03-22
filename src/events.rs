use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use tokio::net::tcp::OwnedWriteHalf;

#[derive(Debug)]
pub enum Event {
    NewConnection {
        ip: SocketAddr,
        out_stream: OwnedWriteHalf,
    },
    NewMessage {
        sender: SocketAddr,
        msg: MsgIn,
    },
    BadMessage(SocketAddr),
    ConnectionDropped(SocketAddr),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgIn {
    pub queue: String,
    pub sender: String,
    pub body: String,
}
