use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::SupplyChain;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Source
/// This object describes the nature and behavior of the entity that is the source
/// of the bid request upstream from the exchange. The primary purpose of this object
/// is to define post-auction or upstream decisioning when the exchange itself does
/// not control the final decision. A common example of this is header bidding, but
/// it can also apply to upstream server entities such as another RTB exchange, a
/// mediation platform, or an ad server combines direct campaigns with 3rd party
/// demand in decisioning.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Source {
    /// Entity responsible for the final impression sale decision, where
    /// 0=exchange, 1=upstream source.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub fd: Option<i64>,
    /// Transaction ID that must be common across all participants in this
    /// bid request (e.g., potentially multiple exchanges).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub tid: Option<String>,
    /// Payment ID chain string containing embedded syntax described in the
    /// TAG Payment ID Protocol v1.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub pchain: Option<String>,
    /// This object represents both the links in the supply chain as well
    /// as an indicator whether or not the supply chain is complete.
    /// Details via the `SupplyChain` object (section 3.2.25).
    pub schain: Option<SupplyChain>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
