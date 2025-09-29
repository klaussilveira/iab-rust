#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Linearity Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum LinearityMode {
    Linear,
    Nonlinear,
    Unknown(i64),
}

impl From<i64> for LinearityMode {
    fn from(value: i64) -> Self {
        match value {
            1 => LinearityMode::Linear,
            2 => LinearityMode::Nonlinear,
            _ => LinearityMode::Unknown(value),
        }
    }
}

impl From<LinearityMode> for i64 {
    fn from(value: LinearityMode) -> Self {
        match value {
            LinearityMode::Linear => 1,
            LinearityMode::Nonlinear => 2,
            LinearityMode::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(LinearityMode);
