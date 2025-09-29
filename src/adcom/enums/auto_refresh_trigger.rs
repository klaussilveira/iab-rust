#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Auto Refresh Triggers
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum AutoRefreshTrigger {
    Unknown(i64),
    UserAction,
    Event,
}

impl From<i64> for AutoRefreshTrigger {
    fn from(value: i64) -> Self {
        match value {
            0 => AutoRefreshTrigger::Unknown(0),
            1 => AutoRefreshTrigger::UserAction,
            2 => AutoRefreshTrigger::Event,
            _ => AutoRefreshTrigger::Unknown(value),
        }
    }
}

impl From<AutoRefreshTrigger> for i64 {
    fn from(value: AutoRefreshTrigger) -> Self {
        match value {
            AutoRefreshTrigger::UserAction => 1,
            AutoRefreshTrigger::Event => 2,
            AutoRefreshTrigger::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(AutoRefreshTrigger);
