mod messages;

use std::{env, error::Error};

use messages::MsgIn;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const BUFF_SIZE: usize = 512 * 1024;
const ACK: &str = "ACK";
const NACK: &str = "NACK";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(handle_incoming(stream));
    }
}

async fn handle_incoming(mut stream: TcpStream) {
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
                let Ok(_msg_in) = serde_json::from_slice::<MsgIn>(&in_buffer[..n]) else {
                    let _ = stream.write(NACK.as_bytes()).await;
                    continue;
                };

                let _ = stream.write(ACK.as_bytes()).await;
            }
        }
    }
}
