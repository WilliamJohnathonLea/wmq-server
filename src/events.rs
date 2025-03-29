use std::net::SocketAddr;

use tokio::net::tcp::OwnedWriteHalf;

use crate::message::Message;

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
    MessageReceived {
        queue_name: String,
        message: Message,
    }
}
