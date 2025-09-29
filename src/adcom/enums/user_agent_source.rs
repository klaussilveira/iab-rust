#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: User Agent Source
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum UserAgentSource {
    Unknown(i64),
    LowEntropy,
    StructuredUserAgentHints,
}

impl From<i64> for UserAgentSource {
    fn from(value: i64) -> Self {
        match value {
            0 => UserAgentSource::Unknown(0),
            1 => UserAgentSource::LowEntropy,
            2 => UserAgentSource::StructuredUserAgentHints,
            _ => UserAgentSource::Unknown(value),
        }
    }
}

impl From<UserAgentSource> for i64 {
    fn from(value: UserAgentSource) -> Self {
        match value {
            UserAgentSource::LowEntropy => 1,
            UserAgentSource::StructuredUserAgentHints => 2,
            UserAgentSource::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(UserAgentSource);
