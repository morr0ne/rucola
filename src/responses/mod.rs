use enum_as_inner::EnumAsInner;
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub mod account;
pub mod listing;
pub mod prefs;
pub mod revision;
pub mod subreddit;
pub mod trophies;
pub mod user_list;

pub use account::Account;
pub use listing::Listing;
pub use prefs::Prefs;
pub use revision::Revision;
pub use subreddit::Subreddit;
pub use trophies::Trophies;
pub use user_list::UserList;

#[derive(Deserialize, EnumAsInner)]
#[serde(tag = "kind", content = "data")]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub enum ThingKind {
    #[serde(rename = "t1")]
    Comment,
    #[serde(rename = "t2")]
    Account(Account),
    #[serde(rename = "t3")]
    Link,
    #[serde(rename = "t4")]
    Message,
    #[serde(rename = "t5")]
    Subreddit(Subreddit),
    #[serde(rename = "t6")]
    Award,
    KarmaList(Vec<KarmaListItem>),
    Listing(Listing<ThingKind>),
    TrophyList,
    UserList(UserList),
    #[serde(rename = "wikipagelisting")]
    WikiPageListing(Vec<String>),
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct KarmaListItem {
    pub comment_karma: i32,
    pub link_karma: i32,
    pub sr: String,
}
