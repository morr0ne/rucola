use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::responses::BasicThing;

pub type Trophies = BasicThing<TrophiesData>;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct TrophiesData {
    pub trophies: Vec<Trophy>,
}

pub type Trophy = BasicThing<TrophyData>;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct TrophyData {
    pub award_id: Option<String>,
    pub description: Option<String>,
    pub granted_at: Option<u64>,
    pub icon_40: String,
    pub icon_70: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}
