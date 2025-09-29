use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_i64_zero;
use super::Bid;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Collection of bids from a specific bidder seat.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct SeatBid {
    /// Array of 1+ Bid objects.
    pub bid: Vec<Bid>,
    /// ID of the buyer seat on whose behalf this bid is made.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub seat: Option<String>,
    /// 0=impressions can be won individually; 1=must be won/lost as group.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub group: Option<i64>,
    /// Placeholder for bidder-specific extensions.
    pub ext: Option<Value>,
}
