use serde::Deserialize;

use crate::message::Message;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Command {
    AssignConsumer { id: String },
    AssignQueue { name: String },
    DeclareQueue { name: String },
    DeleteQueue { name: String },
    SendMessage { queue: String, msg: Message },
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
    fn test_deserialize_assign_queue() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "AssignQueue", "name": "test"}"#;
        let expected = Command::AssignQueue {
            name: "test".into(),
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_declare_queue() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "DeclareQueue", "name": "test"}"#;
        let expected = Command::DeclareQueue {
            name: "test".into(),
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_delete_queue() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "DeleteQueue", "name": "test"}"#;
        let expected = Command::DeleteQueue {
            name: "test".into(),
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_deserialize_send_message() -> Result<(), serde_json::Error> {
        let cmd = r#"{"type": "SendMessage", "queue": "test", "msg": {"sender": "producer1", "body": "hello"}}"#;
        let msg = Message {
            sender: "producer1".into(),
            body: "hello".into(),
        };
        let expected = Command::SendMessage {
            queue: "test".into(),
            msg,
        };
        let actual = serde_json::from_str::<Command>(cmd)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
