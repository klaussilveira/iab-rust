use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::RefSettings;

#[cfg(feature="coercion")]
use crate::json_coercion::AsI64;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Details about ad slots being refreshed automatically.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Refresh {
    /// Array of `RefSettings` objects describing refresh mechanics. Recommended.
    pub refsettings: Option<Vec<RefSettings>>,
    /// Number of times this ad slot refreshed since last page load. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub count: Option<i64>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}
