use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum BreakpointSet {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    #[serde(rename = "@id")]
    pub id: i32,
    #[serde(rename = "@resolved")]
    pub resolved: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl BreakpointSet {
    pub fn from_str(str: &str) -> BreakpointSet {
        if let Ok(response) = from_str(str) {
            return BreakpointSet::Success(response);
        }

        BreakpointSet::Error(from_str(str).unwrap())
    }
}
