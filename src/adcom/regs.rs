use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: Regs
/// This object contains any regulatory conditions in effect for all parties of the
/// transaction with the publisher, advertiser, and nodes of the supply chain being
/// understood as parties to the transaction. Global regulatory attributes should be
/// included here, while regulation specific to a country or region should be included
/// in the subregs ext object (Section 3.4.1).
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Regs {
    /// Flag indicating if this request is subject to the COPPA regulations
    /// established by the USA FTC, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub coppa: Option<i64>,

    /// Global Privacy Platform (GPP) string as defined by the IAB Tech Lab.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub gpp: Option<String>,

    /// Array of applicable GPP Section IDs that inform which sections of the
    /// GPP string are applicable and should be interpreted.
    #[serde(rename="gpp_sid")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsI64>>"))]
    pub gpp_sid: Option<Vec<i64>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
