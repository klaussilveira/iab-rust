use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{link_asset::LinkAsset, asset::Asset};

#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Native
/// This object is used for native ad responses. The native object contains a link object
/// and an array of asset objects. The link object is used to provide a destination link
/// for the native ad if the consumer wishes to visit the advertiser's website, and the
/// assets array is used to provide the constituent assets that comprise the ad.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Native {
    /// Destination link. Refer to Object: LinkAsset.
    pub link: Option<LinkAsset>,

    /// Array of assets comprising the native ad. Refer to Object: Asset.
    pub asset: Option<Vec<Asset>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
