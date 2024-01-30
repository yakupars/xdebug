use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use crate::xml::XdebugError;

#[derive(Debug, Serialize, Deserialize)]
pub enum ContextGet {
    Success(Success),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    #[serde(rename = "@context")]
    pub context: i32,
    pub property: Option<Vec<Property>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@fullname")]
    pub fullname: String,
    #[serde(rename = "@classname")]
    pub classname: Option<String>,
    #[serde(rename = "@page")]
    pub page: Option<i32>,
    #[serde(rename = "@pagesize")]
    pub pagesize: Option<i32>,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@facet")]
    pub facet: Option<String>,
    #[serde(rename = "@size")]
    pub size: Option<i32>,
    #[serde(rename = "@children")]
    pub children: Option<bool>,
    #[serde(rename = "@numchildren")]
    pub numchildren: Option<i32>,
    #[serde(rename = "@key")]
    pub key: Option<String>,
    #[serde(rename = "@address")]
    pub address: Option<String>,
    #[serde(rename = "@encoding")]
    pub encoding: Option<String>,
    pub property: Option<Vec<Property>>,
    #[serde(rename = "$text")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "@command")]
    pub command: String,
    #[serde(rename = "@transaction_id")]
    pub transaction_id: i32,
    error: XdebugError,
}

impl ContextGet {
    pub fn from_str(str: &str) -> ContextGet {
        if let Ok(response) = from_str(str) {
            return ContextGet::Success(response);
        }

        ContextGet::Error(from_str(str).unwrap())
    }
}
