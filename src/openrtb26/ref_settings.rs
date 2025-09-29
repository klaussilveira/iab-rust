use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_reftype_zero;
use crate::adcom;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Information on how often and what triggers an ad slot refresh.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct RefSettings {
    /// Type of declared auto refresh. Refer to `AdCOM 1.0` List: Auto Refresh Triggers. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::AutoRefreshTrigger>>"))]
    #[serde(default="default_optional_reftype_zero")]
    pub reftype: Option<adcom::enums::AutoRefreshTrigger>,
    /// Minimum refresh interval in seconds. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub minint: Option<i64>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}
