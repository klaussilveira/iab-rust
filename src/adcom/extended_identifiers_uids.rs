use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::AgentType;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Extended Identifier UIDs
/// This object contains the unique ID from a source or technology provider specified
/// by the ExtendedIdentifiers.source field.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct ExtendedIdentifiersUids {
    /// Cookie or platform-native identifier.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,

    /// The agent type. Refer to List: Agent Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<AgentType>>"))]
    pub atype: Option<AgentType>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
