use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: Segment
/// Segment objects are essentially key-value pairs that convey specific units of data.
/// The parent Data object is a collection of such values from a given data provider.
/// The specific segment names and value options must be published by the data provider.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Segment {
    /// ID of the data segment specific to the data provider.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Displayable name of the data segment specific to the data provider.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,

    /// String representation of the data segment value.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub value: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
