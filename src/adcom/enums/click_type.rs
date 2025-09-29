#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Click Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum ClickType {
    NonClickable,
    ClickableUnknown,
    ClickableEmbedded,
    ClickableBrowser,
    Unknown(i64),
}

impl From<i64> for ClickType {
    fn from(value: i64) -> Self {
        match value {
            0 => ClickType::NonClickable,
            1 => ClickType::ClickableUnknown,
            2 => ClickType::ClickableEmbedded,
            3 => ClickType::ClickableBrowser,
            _ => ClickType::Unknown(value),
        }
    }
}

impl From<ClickType> for i64 {
    fn from(value: ClickType) -> Self {
        match value {
            ClickType::NonClickable => 0,
            ClickType::ClickableUnknown => 1,
            ClickType::ClickableEmbedded => 2,
            ClickType::ClickableBrowser => 3,
            ClickType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(ClickType);
