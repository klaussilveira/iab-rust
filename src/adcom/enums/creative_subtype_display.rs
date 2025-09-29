#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Creative Subtypes - Display
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum CreativeSubtypeDisplay {
    HTML,
    AMPHTML,
    Image,
    Native,
    Unknown(i64),
}

impl From<i64> for CreativeSubtypeDisplay {
    fn from(value: i64) -> Self {
        match value {
            1 => CreativeSubtypeDisplay::HTML,
            2 => CreativeSubtypeDisplay::AMPHTML,
            3 => CreativeSubtypeDisplay::Image,
            4 => CreativeSubtypeDisplay::Native,
            _ => CreativeSubtypeDisplay::Unknown(value),
        }
    }
}

impl From<CreativeSubtypeDisplay> for i64 {
    fn from(value: CreativeSubtypeDisplay) -> Self {
        match value {
            CreativeSubtypeDisplay::HTML => 1,
            CreativeSubtypeDisplay::AMPHTML => 2,
            CreativeSubtypeDisplay::Image => 3,
            CreativeSubtypeDisplay::Native => 4,
            CreativeSubtypeDisplay::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(CreativeSubtypeDisplay);
