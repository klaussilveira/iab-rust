use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::NativeDataAssetType;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: DataAssetFormat
/// This object is used to provide native asset format specifications for a data element.
/// A data asset is used for all miscellaneous elements such as brand name, ratings,
/// stars, review count, downloads, prices, etc. It is purposefully generic to support
/// native elements not currently contemplated by this specification.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct DataAssetFormat {
    /// The type of data asset requested. Refer to List: Native Data Asset Types.
    #[serde(rename="type")]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<NativeDataAssetType>"))]
    pub r#type: NativeDataAssetType,

    /// The maximum allowed length of the data value.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub len: Option<i64>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
