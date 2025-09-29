#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Expandable Directions
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum ExpandableDirection {
    Left,
    Right,
    Up,
    Down,
    FullScreen,
    Unknown(i64),
}

impl From<i64> for ExpandableDirection {
    fn from(value: i64) -> Self {
        match value {
            1 => ExpandableDirection::Left,
            2 => ExpandableDirection::Right,
            3 => ExpandableDirection::Up,
            4 => ExpandableDirection::Down,
            5 => ExpandableDirection::FullScreen,
            _ => ExpandableDirection::Unknown(value),
        }
    }
}

impl From<ExpandableDirection> for i64 {
    fn from(value: ExpandableDirection) -> Self {
        match value {
            ExpandableDirection::Left => 1,
            ExpandableDirection::Right => 2,
            ExpandableDirection::Up => 3,
            ExpandableDirection::Down => 4,
            ExpandableDirection::FullScreen => 5,
            ExpandableDirection::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(ExpandableDirection);
