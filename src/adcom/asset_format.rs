use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{
    title_asset_format::TitleAssetFormat, image_asset_format::ImageAssetFormat,
    data_asset_format::DataAssetFormat,
};

#[cfg(feature="coercion")]
use crate::json_coercion::AsI64;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: AssetFormat
/// This object represents a permitted format specification for a native asset and is used
/// to indicate the permitted specifications for a native asset of a specific type. The
/// properties of the AssetFormat subtype objects are used to indicate the restrictions
/// on a specific asset of the type implied by the subtype. For example, if the specification
/// is an AssetFormat object that has its img property set to a non-null ImageAssetFormat
/// object, that implies that the specification is for an image asset.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct AssetFormat {
    /// Asset ID, unique within the scope of this placement specification.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub id: Option<i64>,

    /// Indicator of whether or not this asset is required, where 0=no, 1=yes.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub req: i64,

    /// Asset Format Subtype Object that indicates this is specifying a title asset
    /// and provides additional detail as such. Refer to Object: TitleAssetFormat.
    pub title: Option<TitleAssetFormat>,

    /// Asset Format Subtype Object that indicates this is specifying an image asset
    /// and provides additional detail as such. Refer to Object: ImageAssetFormat.
    pub img: Option<ImageAssetFormat>,

    /// Asset Format Subtype Object that indicates this is specifying a video asset
    /// and provides additional detail as such. Refer to Object: VideoPlacement.
    pub video: Option<Value>,

    /// Asset Format Subtype Object that indicates this is specifying a data asset
    /// and provides additional detail as such. Refer to Object: DataAssetFormat.
    pub data: Option<DataAssetFormat>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
