mod messages;

use std::{collections::HashMap, env, error::Error};

use messages::MsgIn;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};

const BUFF_SIZE: usize = 512 * 1024;
const ACK: &str = "ACK";
const NACK: &str = "NACK";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let (tx, rx) = mpsc::channel::<MsgIn>(128);

    tokio::spawn(handle_messages(rx));

    loop {
        let (stream, _) = listener.accept().await?;
        let tx = tx.clone();

        tokio::spawn(handle_incoming(stream, tx));
    }
}

async fn handle_incoming(mut stream: TcpStream, msg_chan: Sender<MsgIn>) {
    let mut in_buffer = [0; BUFF_SIZE];

    loop {
        let n_bytes = stream.read(&mut in_buffer).await;
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
                println!("read {n} bytes");
                let Ok(msg_in) = serde_json::from_slice::<MsgIn>(&in_buffer[..n]) else {
                    let _ = stream.write(NACK.as_bytes()).await;
                    continue;
                };
                let _ = msg_chan.send(msg_in).await;
                let _ = stream.write(ACK.as_bytes()).await;
            }
        }
    }
}

async fn handle_messages(mut msg_chan: Receiver<MsgIn>) {
    let mut queues: Box<HashMap<String, Vec<String>>> = Box::new(HashMap::new());

    println!("starting WMQ Server");
    loop {
        let Some(msg) = msg_chan.recv().await else {
            break;
        };

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
    println!("stopping WMQ Server");
}
