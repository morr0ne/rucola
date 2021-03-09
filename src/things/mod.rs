use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub mod account;
pub mod award;
pub mod comment;
pub mod karma_list;
pub mod link;
pub mod listing;
pub mod prefs;
pub mod revision;
pub mod subreddit;
pub mod user_list;

pub use account::Account;
pub use award::Award;
pub use comment::Comment;
pub use karma_list::KarmaList;
pub use link::Link;
pub use listing::Listing;
pub use prefs::Prefs;
pub use revision::Revision;
pub use subreddit::Subreddit;
pub use user_list::UserList;

#[derive(Deserialize)]
#[serde(tag = "kind", content = "data")]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub enum Thing {
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
    Award(Award),
    KarmaList(KarmaList),
    Listing(Listing<Thing>),
    TrophyList {
        trophies: Vec<Thing>,
    },
    UserList(UserList),
    #[serde(rename = "wikipagelisting")]
    WikiPageListing(Vec<String>),
}
