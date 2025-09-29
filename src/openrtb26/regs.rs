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
/// This object contains any legal, governmental, or industry regulations that the
/// sender deems applicable to the request. See Section 7.5 for more details on
/// the flags supporting Coppa, GDPR and others.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Regs {
    /// Flag indicating if this request is subject to the COPPA regulations
    /// established by the USA FTC, where 0=no, 1=yes.
    /// Refer to Section 7.5 for more information.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub coppa: Option<i64>,
    /// Flag that indicates whether or not the request is subject to GDPR
    /// regulations 0=No, 1=Yes, omission indicates unknown.
    /// Refer to Section 7.5 for more information.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub gdpr: Option<i64>,
    /// Communicates signals regarding consumer privacy under US privacy
    /// regulation. See US Privacy String specifications.
    /// Refer to Section 7.5 for more information.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub us_privacy: Option<String>,
    /// Contains the Global Privacy Platform's consent string.
    /// See the Global Privacy Platform specification for more details.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub gpp: Option<String>,
    /// Array of the section(s) of the string which should be applied for
    /// this transaction. Generally will contain one and only one value,
    /// but there are edge cases where more than one may apply. GPP Section 3
    /// (Header) and 4 (Signal Integrity) do not need to be included.
    /// See the GPP Section Information for more details.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsI64>>"))]
    pub gpp_sid: Option<Vec<i64>>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
