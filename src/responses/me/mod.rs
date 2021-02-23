pub mod karma;
pub mod prefs;
pub mod trophies;
pub use karma::Karma;
pub use prefs::Prefs;
pub use trophies::Trophies;

use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Me {
    pub awardee_karma: u32,
    pub awarder_karma: u32,
    pub can_create_subreddit: bool,
    pub can_edit_name: bool,
    pub coins: u32,
    pub comment_karma: u32,
    pub created: f64,
    pub created_utc: f64,
    pub features: Features,
    pub force_password_reset: bool,
    pub gold_creddits: u32,
    /// I have no idea what the value should be since I don't have a gold subscribtion
    pub gold_expiration: Option<serde_json::Value>,
    pub has_android_subscription: bool,
    pub has_external_account: bool,
    pub has_gold_subscription: bool,
    pub has_ios_subscription: bool,
    pub has_mail: bool,
    pub has_mod_mail: bool,
    pub has_paypal_subscription: bool,
    pub has_stripe_subscription: bool,
    pub has_subscribed: bool,
    pub has_subscribed_to_premium: bool,
    pub has_verified_email: bool,
    pub has_visited_new_profile: bool,
    pub hide_from_robots: bool,
    pub icon_img: String,
    pub inbox_count: u32,
    pub in_beta: bool,
    pub in_chat: Option<bool>,
    pub in_redesign_beta: bool,
    pub id: String,
    pub is_employee: bool,
    pub is_gold: bool,
    pub is_mod: bool,
    pub is_sponsor: bool,
    pub is_suspended: bool,
    pub linked_identities: Vec<String>,
    pub link_karma: u32,
    pub name: String,
    pub new_modmail_exists: Option<bool>,
    pub num_friends: u32,
    pub oauth_client_id: String,
    pub over_18: bool,
    pub password_set: bool,
    pub pref_autoplay: bool,
    pub pref_clickgadget: u32,
    pub pref_geopopular: String,
    pub pref_nightmode: bool,
    pub pref_no_profanity: bool,
    pub pref_show_presence: bool,
    pub pref_show_snoovatar: bool,
    pub pref_show_trending: bool,
    pub pref_show_twitter: bool,
    pub pref_top_karma_subreddits: bool,
    pub pref_video_autoplay: bool,
    pub seen_give_award_tooltip: bool,
    pub seen_layout_switch: bool,
    pub seen_premium_adblock_modal: bool,
    pub seen_redesign_modal: bool,
    pub seen_subreddit_chat_ftux: bool,
    pub snoovatar_img: String,
    pub snoovatar_size: Option<[u32; 2]>,
    pub subreddit: Subreddit,
    pub total_karma: u32,
    /// No idea what this value should be since I've never been suspended
    pub suspension_expiration_utc: Option<serde_json::Value>,
    pub verified: bool,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Features {}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Subreddit {
    pub banner_img: String,
    /// I don't have an banner at the time of writing this, I'll assume it's the same as icon_size
    pub banner_size: Option<[u32; 2]>,
    pub coins: u32,
    /// This returns null on both my accounts so I have no idea what it should be
    pub community_icon: Option<serde_json::Value>,
    pub default_set: bool,
    pub description: String,
    pub disable_contributor_requests: bool,
    pub display_name: String,
    pub display_name_prefixed: String,
    pub free_form_reports: bool,
    /// This returns null on both my accounts so I have no idea what it should be
    pub header_img: Option<serde_json::Value>,
    /// I don't have an header at the time of writing this, I'll assume it's the same as icon_size
    pub header_size: Option<[u32; 2]>,
    pub icon_color: String,
    pub icon_img: String,
    pub icon_size: [u32; 2],
    pub is_default_banner: bool,
    pub is_default_icon: bool,
    pub key_color: String,
    pub link_flair_enabled: bool,
    pub link_flair_position: String,
    pub name: String,
    pub over_18: bool,
    pub previous_names: Vec<String>,
    pub primary_color: String,
    pub public_description: String,
    pub quarantine: bool,
    pub restrict_commenting: bool,
    pub restrict_posting: bool,
    pub show_media: bool,
    pub submit_link_label: String,
    pub submit_text_label: String,
    pub subscribers: u32,
    pub subreddit_type: String,
    pub title: String,
    pub url: String,
    pub user_is_banned: bool,
    pub user_is_contributor: bool,
    pub user_is_muted: bool,
    pub user_is_moderator: bool,
    pub user_is_subscriber: bool,
}
