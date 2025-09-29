#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Content Contexts
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum ContentContext {
    Video,
    Game,
    Music,
    Application,
    Text,
    Other,
    Email,
    Unknown(i64),
}

impl From<i64> for ContentContext {
    fn from(value: i64) -> Self {
        match value {
            1 => ContentContext::Video,
            2 => ContentContext::Game,
            3 => ContentContext::Music,
            4 => ContentContext::Application,
            5 => ContentContext::Text,
            6 => ContentContext::Other,
            7 => ContentContext::Email,
            _ => ContentContext::Unknown(value),
        }
    }
}

impl From<ContentContext> for i64 {
    fn from(value: ContentContext) -> Self {
        match value {
            ContentContext::Video => 1,
            ContentContext::Game => 2,
            ContentContext::Music => 3,
            ContentContext::Application => 4,
            ContentContext::Text => 5,
            ContentContext::Other => 6,
            ContentContext::Email => 7,
            ContentContext::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(ContentContext);
