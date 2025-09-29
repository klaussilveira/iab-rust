use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: TitleAsset
/// This object identifies the native asset as a title asset, which is essentially
/// just a plain text string with specified length.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct TitleAsset {
    /// The text content of the text element.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub text: String,

    /// The length of the contents of the text attribute. RECOMMENDED when len must be
    /// limited in native ad. Refer to AssetTitleFormat.len in the request.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub len: Option<i64>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
