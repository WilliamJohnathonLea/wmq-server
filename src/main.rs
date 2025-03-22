mod events;

use std::{collections::HashMap, env, error::Error, net::SocketAddr};

use events::{Event, MsgIn};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream, tcp::OwnedWriteHalf},
    sync::mpsc::{self, Receiver, Sender},
};

const BUFF_SIZE: usize = 512 * 1024;
// const ACK: &str = "ACK";
// const NACK: &str = "NACK";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let (tx, rx) = mpsc::channel::<Event>(128);

    tokio::spawn(handle_inbound_messages(rx));

    loop {
        let (stream, addr) = listener.accept().await?;
        let tx = tx.clone();

        tokio::spawn(handle_incoming_connection(addr, stream, tx));
    }
}

async fn handle_incoming_connection(ip: SocketAddr, stream: TcpStream, event_chan: Sender<Event>) {
    let (mut rd, wr) = stream.into_split();
    let _ = event_chan
        .send(Event::NewConnection { ip, out_stream: wr })
        .await;

    loop {
        let mut in_buffer = [0; BUFF_SIZE];
        let n_bytes = rd.read(&mut in_buffer).await;
        match n_bytes {
            Err(_) => {
                eprintln!("failed to read from TCP socket");
                let _ = event_chan.send(Event::ConnectionDropped(ip)).await;
                return;
            }
            Ok(0) => {
                println!("socket is closed");
                let _ = event_chan.send(Event::ConnectionDropped(ip)).await;
                return;
            }
            Ok(n) => {
                println!("read {n} bytes");
                let Ok(msg_in) = serde_json::from_slice::<MsgIn>(&in_buffer[..n]) else {
                    continue;
                };
                let _ = event_chan.send(Event::NewMessage(msg_in)).await;
            }
        }
    }
}

async fn handle_inbound_messages(mut event_chan: Receiver<Event>) {
    let mut queues: Box<HashMap<String, Vec<String>>> = Box::new(HashMap::new());
    let mut connections: Box<HashMap<SocketAddr, OwnedWriteHalf>> = Box::new(HashMap::new());

    println!("starting WMQ Server");
    while let Some(event) = event_chan.recv().await {
        match event {
            Event::NewConnection { ip, out_stream } => {
                println!("received new connection from {ip}");
                connections.insert(ip, out_stream);
            }
            Event::NewMessage(msg) => {
                let q = queues.get_mut(&msg.queue);
                match q {
                    None => {
                        queues.insert(msg.queue, vec![msg.body]);
                    }
                    Some(v) => {
                        let mut tmp = vec![msg.body];
                        v.append(&mut tmp);
                    }
                }
            }
            Event::ConnectionDropped(sock_addr) => {
                println!("dropped connection from {sock_addr}");
                connections.remove(&sock_addr);
            }
        }
    }
    println!("stopping WMQ Server");
}
