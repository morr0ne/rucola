use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct UserList {
    pub children: Vec<User>,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct User {
    pub author_flair_css_class: Option<String>,
    pub author_flair_text: Option<String>,
    pub date: f64,
    pub id: String,
    /// This should definatly be an enum
    pub mod_permissions: Option<Vec<String>>,
    pub name: String,
    pub rel_id: String,
}
