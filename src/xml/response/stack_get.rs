use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum StackGet {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    pub stack: Vec<Stack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stack {
    #[serde(rename = "@where")]
    pub where_: String,
    #[serde(rename = "@level")]
    pub level: i32,
    #[serde(rename = "@type")]
    pub type_: String,
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

impl StackGet {
    pub fn from_str(str: &str) -> StackGet {
        if let Ok(response) = from_str(str) {
            return StackGet::Success(response);
        }

        StackGet::Error(from_str(str).unwrap())
    }
}
