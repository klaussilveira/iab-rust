use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: VideoAsset
/// This object is used to provide a video asset used in a native ad. It is recommended
/// that either adm or curl is used to specify a video asset. The other attributes enable
/// the video to be prepared for rendering (e.g., sizes and aspect ratio to scale and
/// position the player appropriately).
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct VideoAsset {
    /// Video markup (e.g., VAST document) for the asset.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub adm: Option<String>,

    /// A URL that returns the video markup (e.g., VAST document) for the asset.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub curl: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
