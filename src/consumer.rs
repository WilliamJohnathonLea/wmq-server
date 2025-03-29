use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::broadcast::Receiver};

use crate::message::Message;

pub struct Consumer {
    out_stream: OwnedWriteHalf,
    queues: Vec<Receiver<Message>>,
}

impl Consumer {
    pub fn new(out_stream: OwnedWriteHalf) -> Self {
        Consumer { out_stream, queues: Vec::new() }
    }

    pub async fn consume(&mut self) {
        
    }
    
    pub fn add_queue(&mut self, rx: Receiver<Message>) {
        self.queues.push(rx);
    }
}
