use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_uasource_zero;
use crate::adcom;
use super::BrandVersion;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Structured user agent information based on User-Agent Client Hints.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct UserAgent {
    /// Array of `BrandVersion` objects identifying browsers/components. Recommended.
    pub browsers: Option<Vec<BrandVersion>>,
    /// `BrandVersion` object identifying platform/OS. Recommended.
    pub platform: Option<BrandVersion>,
    /// 1 if agent prefers 'mobile', 0 if 'desktop'.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub mobile: Option<i64>,
    /// Device's major binary architecture.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub architecture: Option<String>,
    /// Device's bitness.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub bitness: Option<String>,
    /// Device model.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub model: Option<String>,
    /// Source of data used to create this object. Refer to `AdCOM 1.0` List: User-Agent Source.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::UserAgentSource>>"))]
    #[serde(default="default_optional_uasource_zero")]
    pub source: Option<adcom::enums::UserAgentSource>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}
