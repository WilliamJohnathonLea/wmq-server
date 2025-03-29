use std::net::SocketAddr;

use tokio::net::tcp::OwnedWriteHalf;

pub enum Event {
    NewConnection {
        addr: SocketAddr,
        out_stream: OwnedWriteHalf,
    },
    ConsumerAssigned {
        addr: SocketAddr,
        id: String,
    },
    QueueAssigned {
        consumer_id: String,
        queue: String,
    },
    QueueDeclared {
        queue_name: String,
    },
}
