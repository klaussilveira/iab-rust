use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::NativeImageAssetType;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: ImageAsset
/// This object identifies the native asset as a image asset. Image assets are use for
/// such elements as the actual creative images and icons.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct ImageAsset {
    /// A URL that returns the image for the asset.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub url: String,

    /// Width of the image asset in device independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Height of the image asset in device independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// The type of image represented by this asset. Refer to List: Native Image Asset Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<NativeImageAssetType>>"))]
    pub r#type: Option<NativeImageAssetType>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
