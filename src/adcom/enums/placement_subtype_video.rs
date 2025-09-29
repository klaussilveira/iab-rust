#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Placement Subtypes - Video
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum PlacementSubtypeVideo {
    InStream,
    InBanner,
    InArticle,
    InFeed,
    Interstitial,
    Unknown(i64),
}

impl From<i64> for PlacementSubtypeVideo {
    fn from(value: i64) -> Self {
        match value {
            1 => PlacementSubtypeVideo::InStream,
            2 => PlacementSubtypeVideo::InBanner,
            3 => PlacementSubtypeVideo::InArticle,
            4 => PlacementSubtypeVideo::InFeed,
            5 => PlacementSubtypeVideo::Interstitial,
            _ => PlacementSubtypeVideo::Unknown(value),
        }
    }
}

impl From<PlacementSubtypeVideo> for i64 {
    fn from(value: PlacementSubtypeVideo) -> Self {
        match value {
            PlacementSubtypeVideo::InStream => 1,
            PlacementSubtypeVideo::InBanner => 2,
            PlacementSubtypeVideo::InArticle => 3,
            PlacementSubtypeVideo::InFeed => 4,
            PlacementSubtypeVideo::Interstitial => 5,
            PlacementSubtypeVideo::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(PlacementSubtypeVideo);
