use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Identifies a device's browser/component or platform/OS using User-Agent Client Hints.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct BrandVersion {
    /// Brand identifier.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub brand: String,
    /// Sequence of version components.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub version: Option<Vec<String>>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}
