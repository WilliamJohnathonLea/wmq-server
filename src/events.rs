use std::net::SocketAddr;

use tokio::net::tcp::OwnedWriteHalf;

pub enum Event {
    NewConnection {
        addr: SocketAddr,
        out_stream: OwnedWriteHalf,
    },
    ConsumerAssigned {
        addr: SocketAddr,
        queue: String,
    },
}
