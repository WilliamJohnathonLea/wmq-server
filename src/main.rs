use std::{env, error::Error};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

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
    let mut in_buffer = [0; 1024];

    loop {
        let n_bytes = stream.read(&mut in_buffer).await;
        match n_bytes {
            Ok(0) => {
                println!("socket is closed");
                return;
            } // socket closed
            Ok(n) => println!("read {n} bytes"),
            Err(_) => {
                eprintln!("failed to read from TCP socket");
                return;
            }
        }
    }
}
