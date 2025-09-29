#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Display Placement Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DisplayPlacementType {
    InFeed,
    InArticle,
    InContent,
    InRecommendationWidget,
    Unknown(i64),
}

impl From<i64> for DisplayPlacementType {
    fn from(value: i64) -> Self {
        match value {
            1 => DisplayPlacementType::InFeed,
            2 => DisplayPlacementType::InArticle,
            3 => DisplayPlacementType::InContent,
            4 => DisplayPlacementType::InRecommendationWidget,
            _ => DisplayPlacementType::Unknown(value),
        }
    }
}

impl From<DisplayPlacementType> for i64 {
    fn from(value: DisplayPlacementType) -> Self {
        match value {
            DisplayPlacementType::InFeed => 1,
            DisplayPlacementType::InArticle => 2,
            DisplayPlacementType::InContent => 3,
            DisplayPlacementType::InRecommendationWidget => 4,
            DisplayPlacementType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DisplayPlacementType);
