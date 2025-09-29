use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::SupplyChainNode;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Represents the chain of entities involved in the direct flow of payment for inventory.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct SupplyChain {
    /// Flag indicating if chain contains all nodes back to owner (0=no, 1=yes).
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub complete: i64,
    /// Array of `SupplyChainNode` objects in order.
    pub nodes: Vec<SupplyChainNode>,
    /// Version of the supply chain specification.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub ver: String,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
