use enum_as_inner::EnumAsInner;
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub mod account;
pub mod comment;
pub mod karma_list;
pub mod link;
pub mod listing;
pub mod prefs;
pub mod revision;
pub mod subreddit;
pub mod trophies;
pub mod user_list;

pub use account::Account;
pub use comment::Comment;
pub use karma_list::KarmaList;
pub use link::Link;
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
    Comment(Comment),
    #[serde(rename = "t2")]
    Account(Account),
    #[serde(rename = "t3")]
    Link(Link),
    #[serde(rename = "t4")]
    Message,
    #[serde(rename = "t5")]
    Subreddit(Subreddit),
    #[serde(rename = "t6")]
    Award,
    KarmaList(KarmaList),
    Listing(Listing<ThingKind>),
    TrophyList,
    UserList(UserList),
    #[serde(rename = "wikipagelisting")]
    WikiPageListing(Vec<String>),
}
