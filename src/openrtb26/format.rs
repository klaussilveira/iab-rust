use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsI64;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Represents an allowed size (height/width) or Flex Ad parameters.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Format {
    /// Width in DIPS.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,
    /// Height in DIPS.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,
    /// Relative width for ratio size.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wratio: Option<i64>,
    /// Relative height for ratio size.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hratio: Option<i64>,
    /// Minimum width in DIPS for ratio size.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wmin: Option<i64>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
