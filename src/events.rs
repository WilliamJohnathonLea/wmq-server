use tokio::net::tcp::OwnedWriteHalf;

use crate::messages::MsgIn;

pub enum Event {
    NewConsumer {
        id: String,
        queues: Vec<String>,
        out_stream: OwnedWriteHalf,
    },
    NewMessage(MsgIn),
}
