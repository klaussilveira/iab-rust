use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: BrandVersion
/// Further segmentation and identification of the user agent by brand and version.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct BrandVersion {
    /// The brand or maker of the user agent (e.g., Chrome, Safari).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub brand: Option<String>,

    /// The version of the brand (e.g., 105).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub version: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
