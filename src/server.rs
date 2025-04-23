use std::pin::Pin;
use std::task::{Context, Poll};

use tower::Service;

use crate::command::Command;

#[derive(Clone, Copy, Debug)]
pub enum Response {
    Ack,
    Nack,
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Response::Ack => b"ACK",
            Response::Nack => b"NACK",
        }
    }
}

pub struct Server;

impl Service<Vec<u8>> for Server {
    type Response = Response;
    type Error = ();
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Vec<u8>) -> Self::Future {
        match serde_json::from_slice::<Command>(&req) {
            Ok(_) => {
                // Process the request and generate a response
                ResponseFuture::new(Response::Ack)
            }
            Err(_) => ResponseFuture::new(Response::Nack),
        }
    }
}

pub struct ResponseFuture {
    response: Response,
}

impl ResponseFuture {
    pub fn new(response: Response) -> Self {
        ResponseFuture { response }
    }
}

impl Future for ResponseFuture {
    type Output = Result<Response, ()>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(self.response))
    }
}
