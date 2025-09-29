use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::adcom;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Represents a native type impression.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Native {
    /// Request payload complying with Native Ad Specification (JSON encoded string).
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub request: String,
    /// Version of the Dynamic Native Ads API. Highly recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ver: Option<String>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::ApiFramework>>>"))]
    pub api: Option<Vec<adcom::enums::ApiFramework>>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CreativeAttribute>>>"))]
    pub battr: Option<Vec<adcom::enums::CreativeAttribute>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
