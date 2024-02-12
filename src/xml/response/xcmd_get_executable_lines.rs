use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum XcmdGetExecutableLines {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    pub lines: Lines,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lines {
    pub line: Vec<Line>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
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

impl XcmdGetExecutableLines {
    pub fn from_str(str: &str) -> XcmdGetExecutableLines {
        if let Ok(response) = from_str(str) {
            return XcmdGetExecutableLines::Success(response);
        }

        XcmdGetExecutableLines::Error(from_str(str).unwrap())
    }
}
