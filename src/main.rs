mod command;
mod consumer;
mod events;
mod message;

use std::{collections::HashMap, env, error::Error, net::SocketAddr};

use command::Command;
use consumer::Consumer;
use events::Event;
use message::Message;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream, tcp::OwnedWriteHalf},
    sync::{broadcast, mpsc},
};

const BUFF_SIZE: usize = 512 * 1024;

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

async fn handle_events(mut event_chan: mpsc::Receiver<Event>) {
    let mut unassigned_conns = HashMap::<SocketAddr, OwnedWriteHalf>::new();
    let mut consumers = HashMap::<String, Consumer>::new();
    let mut queue_map = HashMap::<String, broadcast::Sender<Message>>::new();

    while let Some(event) = event_chan.recv().await {
        match event {
            Event::NewConnection { addr, out_stream } => {
                unassigned_conns.insert(addr, out_stream);
            }
            Event::ConsumerAssigned { addr, id } => {
                if !unassigned_conns.contains_key(&addr) {
                    eprintln!("connection not found");
                    continue;
                }
                let out_stream = unassigned_conns.remove(&addr).unwrap();
                let consumer = Consumer::new(out_stream);
                consumers.insert(id, consumer);
            }
            Event::ConsumerStarted { id } => {
                // FIXME: removing a consumer allows consumers to have the same id
                //        after starting a consumer.
                if let Some(consumer) = consumers.remove(&id) {
                    tokio::spawn(consumer.consume());
                }
            }
            Event::QueueAssigned { consumer_id, queue } => {
                if let Some(c) = consumers.get_mut(&consumer_id) {
                    if let Some(q) = queue_map.get(&queue) {
                        let rx = q.subscribe();
                        c.add_queue(rx);
                    }
                }
            }
            Event::QueueDeclared { queue_name } => {
                if queue_map.contains_key(&queue_name) {
                    continue;
                }
                let (tx, _) = broadcast::channel::<Message>(20);
                queue_map.insert(queue_name, tx);
            }
            Event::MessageReceived {
                queue_name,
                message,
            } => {
                if let Some(q) = queue_map.get(&queue_name) {
                    let _ = q.send(message);
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
                eprintln!("failed to read from TCP socket");
                return;
            }
            Ok(0) => {
                println!("socket is closed");
                return;
            }
            Ok(n) => {
                let Ok(cmd) = serde_json::from_slice::<Command>(&in_buffer[..n]) else {
                    eprintln!("something went wrong receiving a message");
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
