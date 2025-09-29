use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::extended_identifiers_uids::ExtendedIdentifiersUids;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Extended Identifiers
/// Extended identifiers support in the OpenRTB specification allows buyers to use
/// publisher provided identifiers in the bid request. This object can contain one
/// or more UIDs from a single source or a technology provider. The exchange should
/// ensure that business agreements allow for the sending of this data.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct ExtendedIdentifiers {
    /// Source or technology provider responsible for the set of included IDs.
    /// Expressed as a top-level domain.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub source: String,

    /// Array of extended ID UID objects from the given source.
    /// Refer to Object: Extended Identifier UIDs.
    pub uids: Option<Vec<ExtendedIdentifiersUids>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
