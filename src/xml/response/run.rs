use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum Run {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    #[serde(rename = "@status")]
    pub status: String,
    #[serde(rename = "@reason")]
    pub reason: String,
    pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "@filename")]
    pub filename: String,
    #[serde(rename = "@lineno")]
    pub lineno: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl Run {
    pub fn from_str(str: &str) -> Run {
        if let Ok(response) = from_str(str) {
            return Run::Success(response);
        }

        Run::Error(from_str(str).unwrap())
    }
}
