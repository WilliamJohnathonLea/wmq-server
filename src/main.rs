mod consumer;
mod events;
mod messages;

use std::{collections::HashMap, env, error::Error, net::SocketAddr};

use consumer::Consumer;
use events::Event;
use messages::MsgIn;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::{
        broadcast,
        mpsc::{self, Receiver, Sender},
    },
};

const BUFF_SIZE: usize = 512 * 1024;
// const ACK: &str = "ACK";
// const NACK: &str = "NACK";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let (tx, rx) = mpsc::channel::<Event>(100);

    tokio::spawn(handle_events(rx));

    loop {
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(handle_tcp(addr, stream, tx.clone()));
    }
}

async fn handle_events(mut event_chan: Receiver<Event>) {
    let mut queue_map = HashMap::<String, broadcast::Sender<MsgIn>>::new();

    let (tx, _) = broadcast::channel::<MsgIn>(20);
    while let Some(event) = event_chan.recv().await {
        match event {
            Event::NewConsumer {
                id,
                queues,
                out_stream,
            } => {
                for queue in queues {
                    if !queue_map.contains_key(&queue) {
                        queue_map.insert(queue, tx.clone());
                    }
                }
                let consumer = Consumer::new(out_stream, tx.subscribe());
                tokio::spawn(consumer.consume());
                // FIXME: acks should not be sent to all consumers
                let _ = tx.send(MsgIn::Data {
                    queue: "queue1".to_string(),
                    sender: "producer1".to_string(),
                    body: "hello".to_string(),
                });
            }
            Event::NewMessage(msg_in) => {
                println!("received message: {msg_in:?}");
            }
        }
    }
}

async fn handle_tcp(addr: SocketAddr, stream: TcpStream, event_chan: Sender<Event>) {
    // Splitting the stream for use later
    let (mut rd, wr) = stream.into_split();

    let mut in_buffer = [0; BUFF_SIZE];
    let n_bytes = rd.read(&mut in_buffer).await;
    match n_bytes {
        Ok(n) => {
            let Ok(msg_in) = serde_json::from_slice::<MsgIn>(&in_buffer[..n]) else {
                return;
            };
            match msg_in {
                MsgIn::Consumer { id, queues } => {
                    println!("Consumer {id} connected from {addr}");
                    let _ = event_chan
                        .send(Event::NewConsumer {
                            id,
                            queues,
                            out_stream: wr,
                        })
                        .await;
                }
                MsgIn::Producer { id } => {
                    println!("Producer {id} connected from {addr}");
                }
                _ => {
                    // quit the connection if the message is not a consumer or producer
                    return;
                }
            }
        }
        _ => {
            println!("did not receive any bytes");
            return;
        }
    }

    loop {
        let mut in_buffer = [0; BUFF_SIZE];
        let n_bytes = rd.read(&mut in_buffer).await;
        match n_bytes {
            Err(_) => {
                eprintln!("failed to read from TCP socket");
                return;
            }
            Ok(0) => {
                println!("socket is closed");
                return;
            }
            Ok(n) => {
                let Ok(msg_in) = serde_json::from_slice::<MsgIn>(&in_buffer[..n]) else {
                    continue;
                };
                match msg_in {
                    MsgIn::Data {
                        queue,
                        sender,
                        body,
                    } => {
                        println!("Data from {sender} to {queue}: {body}");
                        let _ = event_chan
                            .send(Event::NewMessage(MsgIn::Data {
                                queue,
                                sender,
                                body,
                            }))
                            .await;
                    }
                    _ => (), // ignore other message types
                }
            }
        }
    }
}
