use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::adcom;
use super::Format;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Represents a banner impression.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Banner {
    /// Array of Format objects representing permitted banner sizes. Recommended if h/w not specified.
    pub format: Option<Vec<Format>>,
    /// Exact width in DIPS. Recommended if no Format objects.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,
    /// Exact height in DIPS. Recommended if no Format objects.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,
    /// Blocked banner ad types (1=XHTML Text, 2=XHTML Banner, 3=JavaScript, 4=iframe).
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsI64>>"))]
    pub btype: Option<Vec<i64>>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CreativeAttribute>>>"))]
    pub battr: Option<Vec<adcom::enums::CreativeAttribute>>,
    /// Ad position on screen. Refer to `AdCOM 1.0` List: Placement Positions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::PlacementPosition>>"))]
    pub pos: Option<adcom::enums::PlacementPosition>,
    /// Content MIME types supported.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub mimes: Option<Vec<String>>,
    /// Indicates if banner is in top frame (0=no, 1=yes).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub topframe: Option<i64>,
    /// Directions banner may expand. Refer to `AdCOM 1.0` List: Expandable Directions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::ExpandableDirection>>>"))]
    pub expdir: Option<Vec<adcom::enums::ExpandableDirection>>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::ApiFramework>>>"))]
    pub api: Option<Vec<adcom::enums::ApiFramework>>,
    /// Unique identifier for this banner object. Recommended for companion ads.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,
    /// Companion banner rendering mode (0=concurrent, 1=end-card). Relevant for companion ads.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub vcm: Option<i64>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
