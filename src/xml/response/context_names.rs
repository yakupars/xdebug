use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum ContextNames {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    pub context: Vec<Context>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@id")]
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl ContextNames {
    pub fn from_str(str: &str) -> ContextNames {
        if let Ok(response) = from_str(str) {
            return ContextNames::Success(response);
        }

        ContextNames::Error(from_str(str).unwrap())
    }
}
