use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{
    title_asset::TitleAsset, image_asset::ImageAsset, video_asset::VideoAsset,
    data_asset::DataAsset, link_asset::LinkAsset,
};

#[cfg(feature="coercion")]
use crate::json_coercion::AsI64;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Asset
/// This object is the container for each asset comprising a native ad. Each asset is
/// of a specific type and to reflect this, one and only one of the subtype objects
/// (i.e., title, img, video, data) must be present; all others should be omitted.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Asset {
    /// The value of AssetFormat.id if this ad references a specific native placement
    /// defined by a Placement object and its structure.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub id: Option<i64>,

    /// Indicates if the asset is required to be displayed, where 0=no, 1=yes.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub req: i64,

    /// Asset Subtype Object that indicates this is a title asset and provides
    /// additional detail as such. Refer to Object: TitleAsset.
    /// * Required if no other asset subtype object is specified.
    pub title: Option<TitleAsset>,

    /// Asset Subtype Object that indicates this is an image asset and provides
    /// additional detail as such. Refer to Object: ImageAsset.
    /// * Required if no other asset subtype object is specified.
    pub image: Option<ImageAsset>,

    /// Asset Subtype Object that indicates this is a video asset and provides
    /// additional detail as such. Refer to Object: VideoAsset.
    /// * Required if no other asset subtype object is specified.
    pub video: Option<VideoAsset>,

    /// Asset Subtype Object that indicates this is a data asset and provides
    /// additional detail as such. Refer to Object: DataAsset.
    /// * Required if no other asset subtype object is specified.
    pub data: Option<DataAsset>,

    /// Asset Subtype Object that indicates this is a link asset and provides
    /// additional detail as such. Refer to Object: LinkAsset.
    /// * Required if no other asset subtype object is specified.
    pub link: Option<LinkAsset>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
