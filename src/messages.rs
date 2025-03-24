use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MsgIn {
    Consumer {
        id: String,
        queues: Vec<String>,
    },
    Producer {
        id: String,
    },
    Data {
        queue: String,
        sender: String,
        body: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialise_consumer_message() -> Result<(), serde_json::Error> {
        let msg = r#"{"type":"Consumer","id":"consumer1","queues":["queue1","queue2"]}"#;
        let msg_in = serde_json::from_str::<MsgIn>(msg)?;
        let expected = MsgIn::Consumer {
            id: "consumer1".to_string(),
            queues: vec!["queue1".to_string(), "queue2".to_string()],
        };
        assert_eq!(msg_in, expected);
        Ok(())
    }

    #[test]
    fn test_deserialise_producer_message() -> Result<(), serde_json::Error> {
        let msg = r#"{"type":"Producer","id":"producer1"}"#;
        let msg_in = serde_json::from_str::<MsgIn>(msg)?;
        let expected = MsgIn::Producer {
            id: "producer1".to_string(),
        };
        assert_eq!(msg_in, expected);
        Ok(())
    }

    #[test]
    fn test_deserialise_data_message() -> Result<(), serde_json::Error> {
        let msg = r#"{"type":"Data","queue":"queue1","sender":"producer1","body":"hello"}"#;
        let msg_in = serde_json::from_str::<MsgIn>(msg)?;
        let expected = MsgIn::Data {
            queue: "queue1".to_string(),
            sender: "producer1".to_string(),
            body: "hello".to_string(),
        };
        assert_eq!(msg_in, expected);
        Ok(())
    }

    #[test]
    fn test_deserialise_bad_message() -> Result<(), serde_json::Error> {
        let msg = r#"{"type":"BadMessage"}"#;
        let msg_in = serde_json::from_str::<MsgIn>(msg);
        assert!(msg_in.is_err());
        Ok(())
    }
}
