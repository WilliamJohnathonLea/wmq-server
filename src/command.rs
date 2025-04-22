use serde::Deserialize;

use crate::message::Message;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Command {
    AssignConsumer {
        id: String,
    },
    StartConsumer {
        id: String,
    },
    AssignProducer {
        id: String,
    },
    AssignQueue {
        consumer_id: String,
        queue: String,
    },
    DeclareQueue {
        name: String,
        size: usize,
    },
    SendMessage {
        queue: String,
        producer_id: String,
        msg: Message,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_assign_consumer() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "AssignConsumer", "id": "my_id"}"#;
        let expected = Command::AssignConsumer { id: "my_id".into() };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_start_consumer() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "StartConsumer", "id": "my_id"}"#;
        let expected = Command::StartConsumer { id: "my_id".into() };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_assign_producer() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "AssignProducer", "id": "my_id"}"#;
        let expected = Command::AssignProducer { id: "my_id".into() };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_assign_queue() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "AssignQueue", "consumer_id": "testId", "queue": "test"}"#;
        let expected = Command::AssignQueue {
            consumer_id: "testId".into(),
            queue: "test".into(),
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_declare_queue() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "DeclareQueue", "name": "test", "size": 100}"#;
        let expected = Command::DeclareQueue {
            name: "test".into(),
            size: 100,
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_send_message() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "SendMessage", "queue": "test", "producer_id": "producer1", "msg": {"sender": "producer1", "body": "hello"}}"#;
        let msg = Message {
            sender: "producer1".into(),
            body: "hello".into(),
        };
        let expected = Command::SendMessage {
            queue: "test".into(),
            producer_id: "producer1".into(),
            msg,
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
