use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::adcom;
use super::UID;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Extended Identifiers support. Contains UIDs from a single source/provider.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct EID {
    /// Canonical domain name of entity that added the ID array element.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub inserter: Option<String>,
    /// Canonical domain of the ID source.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub source: Option<String>,
    /// Technology providing the match method.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub matcher: Option<String>,
    /// Match method used by the matcher. Refer to `AdCOM 1.0` List: ID Match Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::IdMatchMethod>>"))]
    pub mm: Option<adcom::enums::IdMatchMethod>,
    /// Array of extended ID UID objects from the given source.
    pub uids: Option<Vec<UID>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
