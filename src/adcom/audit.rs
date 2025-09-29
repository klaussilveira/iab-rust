use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::AuditStatusCode;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Audit
/// This object is used to convey status or fraud information for an ad, including its
/// intended audience or other ad content related qualifiers. This object is typically
/// used in conjunction with the Ad objects.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Audit {
    /// The audit status of the ad. Refer to List: Audit Status Codes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<AuditStatusCode>>"))]
    pub status: Option<AuditStatusCode>,

    /// Specific human-readable information about reasons for audit status.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub feedback: Option<Vec<String>>,

    /// Unix timestamp of the audit status.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub init: Option<i64>,

    /// Unix timestamp of when audit status last changed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub lastmod: Option<i64>,

    /// Corrigendum object which identifies any errors needing correction.
    pub corr: Option<Value>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
