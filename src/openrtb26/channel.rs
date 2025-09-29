use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Channel {
    /// A unique identifier assigned by the publisher. This may not be a unique identifier across all supply sources.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,
    /// The name of the channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,
    /// The domain of the channel (e.g., "mysite.foo.com").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
