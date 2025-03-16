use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MsgIn {
    pub queue: String,
    pub sender: String,
    pub body: String,
}
