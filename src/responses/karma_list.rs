use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub type KarmaList = Vec<KarmaListItem>;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct KarmaListItem {
    pub comment_karma: i32,
    pub link_karma: i32,
    pub sr: String,
}
