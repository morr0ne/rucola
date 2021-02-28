use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub struct Listing<T> {
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<T>,
    pub count: Option<u32>,
    pub dist: Option<u32>,
    pub limit: Option<String>,
    pub modhash: Option<String>,
    pub show: Option<serde_json::Value>,
}
