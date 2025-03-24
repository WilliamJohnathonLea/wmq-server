use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::broadcast::Receiver};

use crate::messages::MsgIn;

pub struct Consumer {
    out_stream: OwnedWriteHalf,
    rx: Receiver<MsgIn>,
}

impl Consumer {
    pub fn new(out_stream: OwnedWriteHalf, rx: Receiver<MsgIn>) -> Self {
        Consumer { out_stream, rx }
    }

    pub async fn consume(mut self) {
        while let Ok(msg) = self.rx.recv().await {
            println!("sending message to consumer");
            let msg = serde_json::to_string(&msg).unwrap();
            let _ = self.out_stream.write(msg.as_bytes()).await;
        }
    }
}
