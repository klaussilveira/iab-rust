#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Placement Positions
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum PlacementPosition {
    Unknown(i64),
    AboveTheFold,
    BelowTheFold,
    Header,
    Footer,
    Sidebar,
    Fullscreen,
}

impl From<i64> for PlacementPosition {
    fn from(value: i64) -> Self {
        match value {
            0 => PlacementPosition::Unknown(0),
            1 => PlacementPosition::AboveTheFold,
            3 => PlacementPosition::BelowTheFold,
            4 => PlacementPosition::Header,
            5 => PlacementPosition::Footer,
            6 => PlacementPosition::Sidebar,
            7 => PlacementPosition::Fullscreen,
            _ => PlacementPosition::Unknown(value),
        }
    }
}

impl From<PlacementPosition> for i64 {
    fn from(value: PlacementPosition) -> Self {
        match value {
            PlacementPosition::AboveTheFold => 1,
            PlacementPosition::BelowTheFold => 3,
            PlacementPosition::Header => 4,
            PlacementPosition::Footer => 5,
            PlacementPosition::Sidebar => 6,
            PlacementPosition::Fullscreen => 7,
            PlacementPosition::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(PlacementPosition);
