#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Media Ratings
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum MediaRating {
    AllAudiences,
    OverTwelve,
    MatureAudiences,
    Unknown(i64),
}

impl From<i64> for MediaRating {
    fn from(value: i64) -> Self {
        match value {
            1 => MediaRating::AllAudiences,
            2 => MediaRating::OverTwelve,
            3 => MediaRating::MatureAudiences,
            _ => MediaRating::Unknown(value),
        }
    }
}

impl From<MediaRating> for i64 {
    fn from(value: MediaRating) -> Self {
        match value {
            MediaRating::AllAudiences => 1,
            MediaRating::OverTwelve => 2,
            MediaRating::MatureAudiences => 3,
            MediaRating::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(MediaRating);
