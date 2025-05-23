mod command;
mod consumer;
mod events;

use std::{collections::HashMap, env, error::Error, net::SocketAddr};

use command::Command;
use consumer::Consumer;
use events::Event;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, tcp::OwnedWriteHalf},
    sync::{broadcast, mpsc},
};

const BUFF_SIZE: usize = 512 * 1024;
const MAX_QUEUE_SIZE: usize = 5000;

const ACK: &[u8] = "ACK".as_bytes();
const NACK: &[u8] = "NACK".as_bytes();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    colog::init();
    let port = env::var("PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let (tx, rx) = mpsc::channel::<Event>(100);

    tokio::spawn(handle_events(rx));

    loop {
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(handle_tcp(addr, stream, tx.clone()));
    }
}

async fn handle_events(mut event_chan: mpsc::Receiver<Event>) {
    let mut unassigned_conns = HashMap::<SocketAddr, OwnedWriteHalf>::new();
    let mut consumers = HashMap::<String, Consumer>::new();
    let mut producers = HashMap::<String, OwnedWriteHalf>::new();
    let mut queue_map = HashMap::<String, broadcast::Sender<bytes::Bytes>>::new();

    while let Some(event) = event_chan.recv().await {
        match event {
            Event::NewConnection { addr, out_stream } => {
                unassigned_conns.insert(addr, out_stream);
                log::info!("{} connected", addr.ip());
            }
            Event::ConsumerAssigned { addr, id } => {
                let id1 = id.clone();
                if !unassigned_conns.contains_key(&addr) {
                    continue;
                }
                let out_stream = unassigned_conns.remove(&addr).unwrap();
                let consumer = Consumer::new(out_stream);
                consumers.insert(id, consumer);
                log::info!("assigned {} as consumer with ID {}", addr.ip(), id1);
            }
            Event::ConsumerStarted { id } => {
                // FIXME: removing a consumer allows consumers to have the same id
                //        after starting a consumer.
                if let Some(consumer) = consumers.remove(&id) {
                    tokio::spawn(consumer.consume());
                    log::info!("consumer {} started", id);
                }
            }
            Event::ProducerAssigned { addr, id } => {
                let id1 = id.clone();
                if !unassigned_conns.contains_key(&addr) {
                    continue;
                }
                let out_stream = unassigned_conns.remove(&addr).unwrap();
                producers.insert(id, out_stream);
                log::info!("assigned {} as producer with ID {}", addr.ip(), id1);
            }
            Event::QueueAssigned { consumer_id, queue } => {
                if let Some(c) = consumers.get_mut(&consumer_id) {
                    if let Some(q) = queue_map.get(&queue) {
                        let rx = q.subscribe();
                        c.add_queue(rx);
                        log::info!("assigned {} to consumer {}", queue, consumer_id);
                    }
                }
            }
            Event::QueueDeclared { queue_name, size } => {
                let qn = queue_name.clone();
                if queue_map.contains_key(&queue_name) {
                    continue;
                }
                if size > MAX_QUEUE_SIZE {
                    continue;
                }
                let (tx, _) = broadcast::channel::<bytes::Bytes>(size);
                queue_map.insert(queue_name, tx);
                log::info!("queue {} declared", qn);
            }
            Event::MessageReceived {
                queue_name,
                producer_id,
                message,
            } => {
                if let Some(conn) = producers.get_mut(&producer_id) {
                    if let Some(q) = queue_map.get(&queue_name) {
                        match q.send(message) {
                            Ok(_) => {
                                let _ = conn.write(ACK).await;
                                log::info!(
                                    "message successfully received from producer {}",
                                    producer_id
                                );
                            }
                            Err(_) => {
                                let _ = conn
                                    .write(nack_msg("failed to publish message").as_slice())
                                    .await;
                                log::warn!(
                                    "failed to publish message from producer {}",
                                    producer_id
                                );
                            }
                        }
                    } else {
                        let _ = conn
                            .write(nack_msg("queue does not exist").as_slice())
                            .await;
                        log::warn!(
                            "{} tried to publish to undeclared queue {}",
                            producer_id,
                            queue_name
                        );
                    }
                }
            }
        }
    }
}

async fn handle_tcp(addr: SocketAddr, stream: TcpStream, event_chan: mpsc::Sender<Event>) {
    // Splitting the stream for use later
    let (mut rd, wr) = stream.into_split();
    let _ = event_chan
        .send(Event::NewConnection {
            addr,
            out_stream: wr,
        })
        .await;

    loop {
        let mut in_buffer = [0; BUFF_SIZE];
        let n_bytes = rd.read(&mut in_buffer).await;
        match n_bytes {
            Err(_) => {
                log::error!("failed to read from TCP socket");
                return;
            }
            Ok(0) => {
                log::info!("socket is closed");
                return;
            }
            Ok(n) => {
                let Ok(cmd) = serde_json::from_slice::<Command>(&in_buffer[..n]) else {
                    log::error!("something went wrong receiving a message");
                    continue;
                };
                handle_command(cmd, addr, event_chan.clone()).await;
            }
        }
    }
}

async fn handle_command(cmd: Command, addr: SocketAddr, event_chan: mpsc::Sender<Event>) {
    let event = Event::from_network_command(cmd, addr);
    let _ = event_chan.send(event).await;
}

fn nack_msg(msg: &str) -> Vec<u8> {
    [NACK, format!(":{msg}").as_bytes()].concat()
}
