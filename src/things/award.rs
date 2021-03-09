use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Award {
    pub award_id: Option<String>,
    pub description: Option<String>,
    pub granted_at: Option<u64>,
    pub icon_40: String,
    pub icon_70: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}
