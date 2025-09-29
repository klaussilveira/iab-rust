use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

/// Object: LinkAsset
/// This object identifies the native asset as a link asset and is used to define
/// navigation for call-to-action assets. It should be used in place of the deprecated
/// native response's clicktrackers field.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct LinkAsset {
    /// Landing URL of the clickable link.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub url: String,

    /// Fallback URL for deeplink. To be used if the URL given in url is not
    /// supported by the device.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub urlfb: Option<String>,

    /// Array of third-party tracker URLs to be fired on click.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub trkr: Option<Vec<String>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
