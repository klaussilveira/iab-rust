use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_i64_zero;
use super::Deal;

#[cfg(feature="coercion")]
use crate::json_coercion::AsI64;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Private marketplace container for direct deals.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Pmp {
    /// Indicator of auction eligibility (0=all bids, 1=restricted to deals).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub private_auction: Option<i64>,
    /// Array of Deal objects applicable to this impression.
    pub deals: Option<Vec<Deal>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
