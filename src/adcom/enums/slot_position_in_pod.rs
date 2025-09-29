#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Slot Position in Pod
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum SlotPositionInPod {
    Unknown(i64),
    #[default]
    NotApplicable,
    Last,
    First,
    Any,
}

impl From<i64> for SlotPositionInPod {
    fn from(value: i64) -> Self {
        match value {
            -1 => SlotPositionInPod::Unknown(-1),
            0 => SlotPositionInPod::NotApplicable,
            1 => SlotPositionInPod::Last,
            2 => SlotPositionInPod::First,
            3 => SlotPositionInPod::Any,
            _ => SlotPositionInPod::Unknown(value),
        }
    }
}

impl From<SlotPositionInPod> for i64 {
    fn from(value: SlotPositionInPod) -> Self {
        match value {
            SlotPositionInPod::NotApplicable => 0,
            SlotPositionInPod::Last => 1,
            SlotPositionInPod::First => 2,
            SlotPositionInPod::Any => 3,
            SlotPositionInPod::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(SlotPositionInPod);
