use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Prefs {
    pub accept_pms: AcceptPms,
    pub activity_relevant_ads: bool,
    pub allow_clicktracking: bool,
    pub beta: bool,
    pub clickgadget: bool,
    pub collapse_left_bar: bool,
    pub collapse_read_messages: bool,
    pub compress: bool,
    pub default_comment_sort: DefaultCommentSort,
    pub default_theme_sr: Option<serde_json::Value>,
    pub design_beta: bool,
    pub domain_details: bool,
    pub email_comment_reply: bool,
    pub email_chat_request: bool,
    pub email_digests: bool,
    pub email_messages: bool,
    pub email_post_reply: bool,
    pub email_private_message: bool,
    pub email_unsubscribe_all: bool,
    pub email_upvote_comment: bool,
    pub email_upvote_post: bool,
    pub email_user_new_follower: bool,
    pub email_username_mention: bool,
    pub enable_default_themes: bool,
    pub feed_recommendations_enabled: bool,
    pub geopopular: String,
    pub hide_ads: bool,
    pub hide_downs: bool,
    pub hide_from_robots: bool,
    pub hide_ups: bool,
    pub highlight_controversial: bool,
    pub highlight_new_comments: bool,
    pub ignore_suggested_sort: bool,
    pub label_nsfw: bool,
    pub lang: String,
    pub layout: u32,
    pub legacy_search: bool,
    pub live_orangereds: bool,
    pub mark_messages_read: bool,
    pub media: String,
    pub media_preview: String,
    /// Must be within -100 and 100
    pub min_comment_score: i32,
    pub min_link_score: i32,
    pub monitor_mentions: bool,
    pub newwindow: bool,
    pub nightmode: bool,
    pub no_profanity: bool,
    pub numsites: u32,
    pub num_comments: u32,
    pub over_18: bool,
    pub private_feeds: bool,
    pub profile_opt_out: bool,
    pub public_server_seconds: bool,
    pub public_votes: bool,
    pub research: bool,
    pub search_include_over_18: bool,
    pub send_crosspost_messages: bool,
    pub send_welcome_messages: bool,
    pub show_flair: bool,
    pub show_gold_expiration: bool,
    pub show_link_flair: bool,
    pub show_location_based_recommendations: bool,
    pub show_presence: bool,
    pub show_snoovatar: bool,
    pub show_stylesheets: bool,
    pub show_trending: bool,
    pub show_twitter: bool,
    pub store_visits: bool,
    pub survey_last_seen_time: Option<serde_json::Value>,
    pub third_party_data_personalized_ads: bool,
    pub third_party_personalized_ads: bool,
    pub third_party_site_data_personalized_ads: bool,
    pub third_party_site_data_personalized_content: bool,
    pub threaded_messages: bool,
    pub threaded_modmail: bool,
    pub top_karma_subreddits: bool,
    pub use_global_defaults: bool,
    pub video_autoplay: bool,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub enum AcceptPms {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "whitelisted")]
    Whitelisted,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub enum DefaultCommentSort {
    #[serde(rename = "confidence")]
    Confidence,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "controversial")]
    Controversial,
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "qa")]
    Qa,
    #[serde(rename = "live")]
    Live,
}
