use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::asset_format::AssetFormat;

#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: NativeFormat
/// This object refines a display placement to be specifically a native display placement.
/// It serves as the root of a structure that includes the specifications for each of
/// the assets that comprise the native placement.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct NativeFormat {
    /// Array of objects that specify the set of native assets and their permitted
    /// formats. Refer to Object: AssetFormat.
    pub asset: Vec<AssetFormat>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
