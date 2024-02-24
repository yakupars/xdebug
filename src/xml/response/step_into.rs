use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum StepInto {
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
    pub return_value: Option<ReturnValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "@filename")]
    pub filename: String,
    #[serde(rename = "@lineno")]
    pub lineno: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnValue {
    pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "$text")]
    content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl StepInto {
    pub fn from_str(str: &str) -> StepInto {
        if let Ok(response) = from_str(str) {
            return StepInto::Success(response);
        }

        StepInto::Error(from_str(str).unwrap())
    }
}
