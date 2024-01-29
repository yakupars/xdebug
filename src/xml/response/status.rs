use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
struct Success {
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
pub(crate) struct Error {
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

impl Success {
    fn from_str(str: &str) -> Self {
        from_str(str).unwrap()
    }

    fn to_str(&self) -> String {
        to_string(self).unwrap()
    }
}

impl Error {
    fn from_str(str: &str) -> Self {
        from_str(str).unwrap()
    }

    fn to_str(&self) -> String {
        to_string(self).unwrap()
    }
}