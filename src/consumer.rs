use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::broadcast::Receiver};

use futures::stream::{SelectAll, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

pub struct Consumer {
    out_stream: OwnedWriteHalf,
    queues: Vec<Receiver<serde_json::Value>>,
}

impl Consumer {
    pub fn new(out_stream: OwnedWriteHalf) -> Self {
        Consumer {
            out_stream,
            queues: Vec::new(),
        }
    }

    pub async fn consume(mut self) {
        let mut streams: SelectAll<_> = self.queues.into_iter().map(BroadcastStream::new).collect();

        while let Some(result) = streams.next().await {
            if let Ok(msg) = result {
                if let Ok(json) = serde_json::to_string(&msg) {
                    _ = self.out_stream.write(json.as_bytes()).await;
                }
            }
        }
    }

    pub fn add_queue(&mut self, rx: Receiver<serde_json::Value>) {
        self.queues.push(rx);
    }
}
