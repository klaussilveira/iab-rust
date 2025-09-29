use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::NativeDataAssetType;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: DataAsset
/// This object identifies the native asset as a data asset. A data asset is used for all
/// miscellaneous elements such as brand name, ratings, stars, review count, downloads,
/// price, counts, etc. It is purposefully generic to support native elements not currently
/// contemplated by this specification.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct DataAsset {
    /// A formatted string of data to be displayed (e.g., "5 stars", "3.4 stars out of 5",
    /// "$10", etc.).
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub value: String,

    /// The length of the value contents. This length should conform to recommendations
    /// provided in List: Native Data Asset Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub len: Option<i64>,

    /// The type of data represented by this asset. Refer to List: Native Data Asset Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<NativeDataAssetType>>"))]
    pub r#type: Option<NativeDataAssetType>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
