#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Native Image Asset Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum NativeImageAssetType {
    Icon,
    Logo,
    Main,
    Unknown(i64),
}

impl From<i64> for NativeImageAssetType {
    fn from(value: i64) -> Self {
        match value {
            1 => NativeImageAssetType::Icon,
            2 => NativeImageAssetType::Logo,
            3 => NativeImageAssetType::Main,
            _ => NativeImageAssetType::Unknown(value),
        }
    }
}

impl From<NativeImageAssetType> for i64 {
    fn from(value: NativeImageAssetType) -> Self {
        match value {
            NativeImageAssetType::Icon => 1,
            NativeImageAssetType::Logo => 2,
            NativeImageAssetType::Main => 3,
            NativeImageAssetType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(NativeImageAssetType);
