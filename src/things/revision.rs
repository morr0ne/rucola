use super::{Listing, Thing};
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub type Revision = Listing<RevisionData>;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct RevisionData {
    pub author: Option<Thing>,
    pub id: String,
    pub page: String,
    pub reason: Option<String>,
    pub revision_hidden: bool,
    pub timestamp: u32,
}
