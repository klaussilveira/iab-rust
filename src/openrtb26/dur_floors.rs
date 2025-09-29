use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_f64_zero;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsI64, AsF64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Allows specifying price floors for video/audio creatives based on duration ranges.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct DurFloors {
    /// Low end of duration range (seconds). If missing, unbounded.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub mindur: Option<i64>,
    /// High end of duration range (seconds). If missing, unbounded.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxdur: Option<i64>,
    /// Minimum bid (CPM) for this duration range. Defaults to Imp.bidfloor if outside ranges.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    #[serde(default="default_optional_f64_zero")]
    pub bidfloor: Option<f64>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}
