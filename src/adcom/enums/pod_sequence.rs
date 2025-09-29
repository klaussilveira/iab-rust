#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Pod Sequence
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum PodSequence {
    LastPosition,
    #[default]
    AnyPosition,
    FirstPosition,
    Unknown(i64),
}

impl From<i64> for PodSequence {
    fn from(value: i64) -> Self {
        match value {
            -1 => PodSequence::LastPosition,
            0 => PodSequence::AnyPosition,
            1 => PodSequence::FirstPosition,
            _ => PodSequence::Unknown(value),
        }
    }
}

impl From<PodSequence> for i64 {
    fn from(value: PodSequence) -> Self {
        match value {
            PodSequence::LastPosition => -1,
            PodSequence::AnyPosition => 0,
            PodSequence::FirstPosition => 1,
            PodSequence::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(PodSequence);
