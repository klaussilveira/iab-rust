#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Start Delay Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum StartDelayMode {
    PreRoll,
    MidRoll,
    PostRoll,
    Unknown(i64),
}

impl From<i64> for StartDelayMode {
    fn from(value: i64) -> Self {
        match value {
            -1 => StartDelayMode::MidRoll,
            -2 => StartDelayMode::PostRoll,
            0 => StartDelayMode::PreRoll,
            _ => StartDelayMode::Unknown(value),
        }
    }
}

impl From<StartDelayMode> for i64 {
    fn from(value: StartDelayMode) -> Self {
        match value {
            StartDelayMode::PreRoll => 0,
            StartDelayMode::MidRoll => -1,
            StartDelayMode::PostRoll => -2,
            StartDelayMode::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(StartDelayMode);
