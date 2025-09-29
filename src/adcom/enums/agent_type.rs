#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Agent Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum AgentType {
    AdServer,
    Exchange,
    Dsp,
    AdVerifier,
    CreativeAgency,
    Unknown(i64),
}

impl From<i64> for AgentType {
    fn from(value: i64) -> Self {
        match value {
            1 => AgentType::AdServer,
            2 => AgentType::Exchange,
            3 => AgentType::Dsp,
            4 => AgentType::AdVerifier,
            5 => AgentType::CreativeAgency,
            _ => AgentType::Unknown(value),
        }
    }
}

impl From<AgentType> for i64 {
    fn from(value: AgentType) -> Self {
        match value {
            AgentType::AdServer => 1,
            AgentType::Exchange => 2,
            AgentType::Dsp => 3,
            AgentType::AdVerifier => 4,
            AgentType::CreativeAgency => 5,
            AgentType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(AgentType);
