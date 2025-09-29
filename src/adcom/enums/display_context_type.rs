#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Display Context Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DisplayContextType {
    ContentCentric,
    ArticleContent,
    VideoContent,
    AudioContent,
    ImageContent,
    UserGeneratedContent,
    SocialCentric,
    EmailContent,
    ChatIMContent,
    ProductContext,
    AppStoreMarketplace,
    ProductReviews,
    Unknown(i64),
}

impl From<i64> for DisplayContextType {
    fn from(value: i64) -> Self {
        match value {
            10 => DisplayContextType::ContentCentric,
            11 => DisplayContextType::ArticleContent,
            12 => DisplayContextType::VideoContent,
            13 => DisplayContextType::AudioContent,
            14 => DisplayContextType::ImageContent,
            15 => DisplayContextType::UserGeneratedContent,
            20 => DisplayContextType::SocialCentric,
            21 => DisplayContextType::EmailContent,
            22 => DisplayContextType::ChatIMContent,
            30 => DisplayContextType::ProductContext,
            31 => DisplayContextType::AppStoreMarketplace,
            32 => DisplayContextType::ProductReviews,
            _ => DisplayContextType::Unknown(value),
        }
    }
}

impl From<DisplayContextType> for i64 {
    fn from(value: DisplayContextType) -> Self {
        match value {
            DisplayContextType::ContentCentric => 10,
            DisplayContextType::ArticleContent => 11,
            DisplayContextType::VideoContent => 12,
            DisplayContextType::AudioContent => 13,
            DisplayContextType::ImageContent => 14,
            DisplayContextType::UserGeneratedContent => 15,
            DisplayContextType::SocialCentric => 20,
            DisplayContextType::EmailContent => 21,
            DisplayContextType::ChatIMContent => 22,
            DisplayContextType::ProductContext => 30,
            DisplayContextType::AppStoreMarketplace => 31,
            DisplayContextType::ProductReviews => 32,
            DisplayContextType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DisplayContextType);
