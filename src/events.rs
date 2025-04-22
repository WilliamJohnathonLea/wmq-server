use std::net::SocketAddr;

use tokio::net::tcp::OwnedWriteHalf;

use crate::command::Command;

pub enum Event {
    NewConnection {
        addr: SocketAddr,
        out_stream: OwnedWriteHalf,
    },
    ConsumerAssigned {
        addr: SocketAddr,
        id: String,
    },
    ConsumerStarted {
        id: String,
    },
    ProducerAssigned {
        addr: SocketAddr,
        id: String,
    },
    QueueAssigned {
        consumer_id: String,
        queue: String,
    },
    QueueDeclared {
        queue_name: String,
        size: usize,
    },
    MessageReceived {
        queue_name: String,
        producer_id: String,
        message: serde_json::Value,
    },
}

impl Event {
    pub fn from_network_command(cmd: Command, addr: SocketAddr) -> Event {
        match cmd {
            Command::AssignConsumer { id } => Event::ConsumerAssigned { addr, id },
            Command::StartConsumer { id } => Event::ConsumerStarted { id },
            Command::AssignProducer { id } => Event::ProducerAssigned { addr, id },
            Command::AssignQueue { consumer_id, queue } => {
                Event::QueueAssigned { consumer_id, queue }
            }
            Command::DeclareQueue { name, size } => Event::QueueDeclared {
                queue_name: name,
                size,
            },
            Command::SendMessage {
                queue,
                producer_id,
                msg,
            } => Event::MessageReceived {
                queue_name: queue,
                producer_id,
                message: msg,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use crate::command::Command;

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
    fn consumer_started_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let id = "test".to_string();
        let cmd = Command::StartConsumer { id: id.clone() };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::ConsumerStarted { id: actual_id } => {
                assert_eq!(id, actual_id);
                Ok(())
            }
            _ => Err(()),
        }
    }

    #[test]
    fn producer_assigned_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let id = "test".to_string();
        let cmd = Command::AssignProducer { id: id.clone() };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::ProducerAssigned {
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
            size: 100,
        };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::QueueDeclared {
                queue_name: actual_queue,
                size: actual_size,
            } => {
                assert_eq!(queue, actual_queue);
                assert_eq!(100, actual_size);
                Ok(())
            }
            _ => Err(()),
        }
    }

    #[test]
    fn message_received_from_command() -> Result<(), ()> {
        let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080).into();
        let queue = "test_queue".to_string();
        let producer_id = "producer1".to_string();
        let msg = serde_json::Value::String("hello".to_string());
        let cmd = Command::SendMessage {
            queue: queue.clone(),
            producer_id: producer_id.clone(),
            msg: msg.clone(),
        };

        let actual = Event::from_network_command(cmd, addr);

        match actual {
            Event::MessageReceived {
                queue_name: actual_queue,
                producer_id: actual_producer_id,
                message: actual_message,
            } => {
                assert_eq!(queue, actual_queue);
                assert_eq!(producer_id, actual_producer_id);
                assert_eq!(msg, actual_message);
                Ok(())
            }
            _ => Err(()),
        }
    }
}
