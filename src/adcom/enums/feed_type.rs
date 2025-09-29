#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Feed Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum FeedType {
    MusicService,
    FMRadioBroadcast,
    AMRadioBroadcast,
    Podcast,
    Unknown(i64),
}

impl From<i64> for FeedType {
    fn from(value: i64) -> Self {
        match value {
            1 => FeedType::MusicService,
            2 => FeedType::FMRadioBroadcast,
            3 => FeedType::AMRadioBroadcast,
            4 => FeedType::Podcast,
            _ => FeedType::Unknown(value),
        }
    }
}

impl From<FeedType> for i64 {
    fn from(value: FeedType) -> Self {
        match value {
            FeedType::MusicService => 1,
            FeedType::FMRadioBroadcast => 2,
            FeedType::AMRadioBroadcast => 3,
            FeedType::Podcast => 4,
            FeedType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(FeedType);
