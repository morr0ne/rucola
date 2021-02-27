pub mod me;
pub mod subreddits;

use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub struct BasicThing<T> {
    pub kind: String,
    pub data: T,
}

pub type Listing<T> = BasicThing<ListingData<T>>;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub struct ListingData<T> {
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<BasicThing<T>>,
    pub count: Option<u32>,
    pub dist: Option<u32>,
    pub limit: Option<String>,
    pub modhash: Option<String>,
    pub show: Option<serde_json::Value>,
}
