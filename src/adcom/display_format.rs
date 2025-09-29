use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsI64;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: DisplayFormat
/// This object represents an allowed set of parameters for a banner display ad and often
/// appears as an array when multiple sizes are permitted.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct DisplayFormat {
    /// Absolute width of the creative in units specified by DisplayPlacement.unit.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Absolute height of the creative in units specified by DisplayPlacement.unit.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// Relative width of the creative when expressing size as a ratio, in which case
    /// both wratio and hratio must be included.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wratio: Option<i64>,

    /// Relative height of the creative when expressing size as a ratio, in which case
    /// both wratio and hratio must be included.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hratio: Option<i64>,

    /// Minimum requested width of the creative in units specified by DisplayPlacement.unit.
    /// This field is optional. If included, it indicates that the creative is a native ad
    /// element that should scale between the size indicated by this field and the w/h fields.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wmin: Option<i64>,

    /// Minimum requested height of the creative in units specified by DisplayPlacement.unit.
    /// This field is optional. If included, it indicates that the creative is a native ad
    /// element that should scale between the size indicated by this field and the w/h fields.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hmin: Option<i64>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
