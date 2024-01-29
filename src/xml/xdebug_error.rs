use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XdebugError {
    #[serde(rename = "@code")]
    code: i32,
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    #[serde(rename = "$text")]
    content: String,
}