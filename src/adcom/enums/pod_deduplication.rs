#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Pod Deduplication Settings
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum PodDeduplication {
    NoRestrictions,
    ExcludeAds,
    ExcludeAdvertisers,
    Unknown(i64),
}

impl From<i64> for PodDeduplication {
    fn from(value: i64) -> Self {
        match value {
            0 => PodDeduplication::NoRestrictions,
            1 => PodDeduplication::ExcludeAds,
            2 => PodDeduplication::ExcludeAdvertisers,
            _ => PodDeduplication::Unknown(value),
        }
    }
}

impl From<PodDeduplication> for i64 {
    fn from(value: PodDeduplication) -> Self {
        match value {
            PodDeduplication::NoRestrictions => 0,
            PodDeduplication::ExcludeAds => 1,
            PodDeduplication::ExcludeAdvertisers => 2,
            PodDeduplication::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(PodDeduplication);
