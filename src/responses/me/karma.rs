use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Karma {
    pub data: Vec<Data>,
    pub kind: String,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Data {
    pub comment_karma: i32,
    pub link_karma: i32,
    pub sr: String,
}
