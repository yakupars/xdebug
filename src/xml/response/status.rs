use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl Status {
    pub fn from_str(str: &str) -> Status {
        if let Ok(response) = from_str(str) {
            return Status::Success(response);
        }

        Status::Error(from_str(str).unwrap())
    }
}
