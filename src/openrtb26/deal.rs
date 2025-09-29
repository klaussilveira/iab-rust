use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::{default_optional_string_usd, default_optional_i64_zero};
use super::DurFloors;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsF64, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Constitutes a specific deal struck between a buyer and a seller.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Deal {
    /// Unique identifier for the direct deal.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,
    /// Minimum bid for this impression expressed in CPM.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub bidfloor: Option<f64>,
    /// Currency for bidfloor (ISO-4217 alpha codes).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    #[serde(default="default_optional_string_usd")]
    pub bidfloorcur: Option<String>,
    /// Optional override of overall auction type (1=First Price, 2=Second Price Plus, 3=Deal Price).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub at: Option<i64>,
    /// Allowed list of buyer seats for this deal.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub wseat: Option<Vec<String>>,
    /// Array of advertiser domains allowed for this deal.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub wadomain: Option<Vec<String>>,
    /// Indicates if deal is 'guaranteed' (0=not guaranteed, 1=guaranteed).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub guar: Option<i64>,
    /// Minimum CPM per second for video/audio opportunities.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub mincpmpersec: Option<f64>,
    /// Container for floor price by duration information (video/audio).
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
