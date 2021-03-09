use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Link {
    pub all_awardings: Vec<Awarder>,
    pub allow_live_comments: bool,
    pub approved_at_utc: Option<f64>,
    pub approved_by: Option<serde_json::Value>,
    pub author_flair_background_color: Option<serde_json::Value>,
    pub author_flair_css_class: Option<String>,
    pub author_flair_richtext: Vec<serde_json::Value>,
    pub author_flair_template_id: Option<String>,
    pub author_flair_type: String,
    pub author_fullname: String,
    pub author_premium: bool,
    pub archived: bool,
    pub banned_at_utc: Option<f64>,
    pub banned_by: Option<serde_json::Value>,
    pub can_mod_post: bool,
    pub category: Option<serde_json::Value>,
    pub clicked: bool,
    pub content_categories: Option<serde_json::Value>,
    pub created: f64,
    pub domain: String,
    pub downs: u32,
    pub edited: bool,
    pub gilded: i32,
    pub gildings: Gildings,
    pub hidden: bool,
    pub hide_score: bool,
    pub is_crosspostable: bool,
    pub is_meta: bool,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool,
    pub is_self: bool,
    pub likes: Option<serde_json::Value>,
    pub link_flair_css_class: Option<String>,
    pub link_flair_richtext: Vec<String>,
    pub link_flair_text: Option<String>,
    pub link_flair_text_color: String,
    pub link_flair_type: String,
    pub media_embed: MediaEmbed,
    pub mod_note: Option<String>,
    pub mod_reason_title: Option<String>,
    pub name: String,
    pub no_follow: bool,
    pub over_18: bool,
    pub pinned: bool,
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
    pub pwls: Option<u32>,
    pub quarantine: bool,
    pub removed_by_category: Option<serde_json::Value>,
    pub saved: bool,
    pub score: i32,
    pub secure_media: Option<SecureMedia>,
    pub secure_media_embed: SecureMediaEmbed,
    pub selftext: Option<String>,
    pub selftext_html: Option<String>,
    pub subreddit: String,
    pub subreddit_name_prefixed: String,
    /// This should be an enum
    pub subreddit_type: String,
    pub suggested_sort: Option<String>,
    pub thumbnail: String,
    pub thumbnail_height: Option<u32>,
    pub thumbnail_width: Option<u32>,
    pub title: String,
    pub top_awarded_type: Option<serde_json::Value>,
    pub total_awards_received: u32,
    pub user_reports: Vec<serde_json::Value>,
    pub ups: i32,
    pub upvote_ratio: f32,
    pub url_overridden_by_dest: Option<String>,
    pub view_count: Option<serde_json::Value>,
    pub wls: Option<u32>,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Awarder {
    /// Could probably be an enum
    pub award_sub_type: String,
    pub coin_price: u32,
    pub coin_reward: u32,
    pub days_of_drip_extension: u32,
    pub days_of_premium: u32,
    pub giver_coin_reward: u32,
    pub icon_url: String,
    pub id: String,
    pub is_new: bool,
    pub penny_donate: Option<u32>,
    pub subreddit_id: Option<String>,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct MediaEmbed {}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SecureMedia {
    pub reddit_video: RedditVideo,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct RedditVideo {
    pub bitrate_kbps: u32,
    pub dash_url: String,
    pub duration: u32,
    pub fallback_url: String,
    pub height: u32,
    pub hls_url: String,
    pub is_gif: bool,
    pub scrubber_media_url: String,
    /// Could probably be an enum
    pub transcoding_status: String,
    pub width: u32,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SecureMediaEmbed {}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Gildings {}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Preview {
    pub enable: Option<bool>,
    pub images: Vec<Image>,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Image {
    pub id: String,
    pub resolutions: Vec<Source>,
    pub source: Source,
    pub variants: Variants,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Source {
    pub url: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Variants {}
