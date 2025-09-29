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

/// Object: ImageAssetFormat
/// This object is used to provide native asset format specifications for an image.
/// Image asset formats must include a supported mime type and at least one of
/// w+h or wmin+hmin or w,h,wmin,hmin for strict or flexible size requirements.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct ImageAssetFormat {
    /// The type of image asset requested. Refer to List: Native Image Asset Types.
    #[serde(rename="type")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<NativeImageAssetType>>"))]
    pub r#type: Option<NativeImageAssetType>,

    /// Array of supported mime types (e.g., "image/jpeg", "image/gif").
    /// If omitted, assume all types are allowed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub mime: Option<Vec<String>>,

    /// Recommended absolute width of the image asset in device independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Recommended absolute height of the image asset in device independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// Minimum absolute width of the image asset in device independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wmin: Option<i64>,

    /// Minimum absolute height of the image asset in device independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hmin: Option<i64>,

    /// Recommended relative width of the image asset when expressing flexible sizes.
    /// Note that mixing absolute and relative sizes is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wratio: Option<i64>,

    /// Recommended relative height of the image asset when expressing flexible sizes.
    /// Note that mixing absolute and relative sizes is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hratio: Option<i64>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
