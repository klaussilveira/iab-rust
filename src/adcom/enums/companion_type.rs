#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Companion Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum CompanionType {
    ConcurrentDisplay,
    EndCard,
    Unknown(i64),
}

impl From<i64> for CompanionType {
    fn from(value: i64) -> Self {
        match value {
            1 => CompanionType::ConcurrentDisplay,
            2 => CompanionType::EndCard,
            _ => CompanionType::Unknown(value),
        }
    }
}

impl From<CompanionType> for i64 {
    fn from(value: CompanionType) -> Self {
        match value {
            CompanionType::ConcurrentDisplay => 1,
            CompanionType::EndCard => 2,
            CompanionType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(CompanionType);
