use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialise_data_message() -> Result<(), serde_json::Error> {
        let msg = r#"{"sender":"producer1","body":"hello"}"#;
        let msg_in = serde_json::from_str::<Message>(msg)?;
        let expected = Message {
            sender: "producer1".to_string(),
            body: "hello".to_string(),
        };
        assert_eq!(msg_in, expected);
        Ok(())
    }

    #[test]
    fn test_deserialise_bad_message() -> Result<(), serde_json::Error> {
        let msg = r#"{"type":"BadMessage"}"#;
        let msg_in = serde_json::from_str::<Message>(msg);
        assert!(msg_in.is_err());
        Ok(())
    }
}
