pub mod me;

use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub struct Thing<T> {
    pub kind: String,
    pub data: T,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub struct Listing<T> {
    pub before: String,
    pub after: String,
    pub modhash: String,
    pub children: Vec<Thing<T>>,
}
