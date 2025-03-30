use std::net::SocketAddr;

use tokio::net::tcp::OwnedWriteHalf;

use crate::{command::Command, message::Message};

pub enum Event {
    NewConnection {
        addr: SocketAddr,
        out_stream: OwnedWriteHalf,
    },
    ConsumerAssigned {
        addr: SocketAddr,
        id: String,
    },
    QueueAssigned {
        consumer_id: String,
        queue: String,
    },
    QueueDeclared {
        queue_name: String,
    },
    MessageReceived {
        queue_name: String,
        message: Message,
    },
}

impl Event {
    pub fn from_network_command(cmd: Command, addr: SocketAddr) -> Event {
        match cmd {
            Command::AssignConsumer { id } => Event::ConsumerAssigned { addr, id },
            Command::AssignQueue { consumer_id, queue } => {
                Event::QueueAssigned { consumer_id, queue }
            }
            Command::DeclareQueue { name } => Event::QueueDeclared { queue_name: name },
            Command::DeleteQueue { name } => todo!(),
            Command::SendMessage { queue, msg } => Event::MessageReceived {
                queue_name: queue,
                message: msg,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use crate::{command::Command, message::Message};

    use super::Event;

    #[test]
    fn consumer_assigned_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let id = "test".to_string();
        let cmd = Command::AssignConsumer { id: id.clone() };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::ConsumerAssigned {
                addr: actual_addr,
                id: actual_id,
            } => {
                assert_eq!(addr, actual_addr);
                assert_eq!(id, actual_id);
                Ok(())
            }
            _ => Err(()),
        }
    }

    #[test]
    fn queue_assigned_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let id = "test".to_string();
        let queue = "test_queue".to_string();
        let cmd = Command::AssignQueue {
            consumer_id: id.clone(),
            queue: queue.clone(),
        };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::QueueAssigned {
                consumer_id: actual_consumer_id,
                queue: actual_queue,
            } => {
                assert_eq!(id, actual_consumer_id);
                assert_eq!(queue, actual_queue);
                Ok(())
            }
            _ => Err(()),
        }
    }

    #[test]
    fn queue_declared_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let queue = "test_queue".to_string();
        let cmd = Command::DeclareQueue {
            name: queue.clone(),
        };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::QueueDeclared {
                queue_name: actual_queue,
            } => {
                assert_eq!(queue, actual_queue);
                Ok(())
            }
            _ => Err(()),
        }
    }

    #[test]
    fn message_received_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let queue = "test_queue".to_string();
        let msg = Message {
            sender: "producer1".into(),
            body: "hello".into(),
        };
        let cmd = Command::SendMessage {
            queue: queue.clone(),
            msg: msg.clone(),
        };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::MessageReceived {
                queue_name: actual_queue,
                message: actual_message,
            } => {
                assert_eq!(queue, actual_queue);
                assert_eq!(msg, actual_message);
                Ok(())
            }
            _ => Err(()),
        }
    }
}
