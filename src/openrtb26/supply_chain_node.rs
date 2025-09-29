use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Defines the identity of an entity participating in the supply chain.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct SupplyChainNode {
    /// Canonical domain name of advertising system.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub asi: String,
    /// Seller/reseller account ID within the advertising system.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub sid: String,
    /// `OpenRTB` `RequestId` issued by this seller.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub rid: Option<String>,
    /// Name of the company paid for inventory under sid. Optional.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,
    /// Business domain name of the entity. Optional.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,
    /// Indicates if node involved in payment flow (1=yes, 0=no). Should be 1 for v1.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hp: Option<i64>,
    /// Placeholder for advertising-system specific extensions.
    pub ext: Option<Value>,
}
