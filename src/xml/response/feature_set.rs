use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum FeatureSet {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    #[serde(rename = "@feature")]
    pub feature: String,
    #[serde(rename = "@success")]
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl FeatureSet {
    pub fn from_str(str: &str) -> FeatureSet {
        if let Ok(response) = from_str(str) {
            return FeatureSet::Success(response);
        }

        FeatureSet::Error(from_str(str).unwrap())
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