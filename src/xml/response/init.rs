use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Init {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@xmlns:xdebug")]
    xmlns_xdebug: String,
    #[serde(rename = "@fileuri")]
    fileuri: String,
    #[serde(rename = "@language")]
    language: String,
    #[serde(rename = "@language_version")]
    xdebug_language_version: String,
    #[serde(rename = "@protocol_version")]
    protocol_version: String,
    #[serde(rename = "@appid")]
    appid: String,
    #[serde(rename = "@idekey")]
    idekey: Option<String>,
    engine: Engine,
    author: Author,
    url: Url,
    copyright: Copyright,
}

#[derive(Debug, Serialize, Deserialize)]
struct Engine {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "$text")]
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    #[serde(rename = "$text")]
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Url {
    #[serde(rename = "$text")]
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Copyright {
    #[serde(rename = "$text")]
    content: String,
}

impl Init {
    pub fn from_str(str: &str) -> Self {
        from_str(str).unwrap()
    }
}
