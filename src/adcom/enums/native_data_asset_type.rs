#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Native Data Asset Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum NativeDataAssetType {
    Unknown(i64),
    Sponsored,
    Desc,
    Rating,
    Likes,
    Downloads,
    Price,
    SalePrice,
    Phone,
    Address,
    Desc2,
    DisplayUrl,
    CtatextButtonText,
}

impl From<i64> for NativeDataAssetType {
    fn from(value: i64) -> Self {
        match value {
            0 => NativeDataAssetType::Unknown(0),
            1 => NativeDataAssetType::Sponsored,
            2 => NativeDataAssetType::Desc,
            3 => NativeDataAssetType::Rating,
            4 => NativeDataAssetType::Likes,
            5 => NativeDataAssetType::Downloads,
            6 => NativeDataAssetType::Price,
            7 => NativeDataAssetType::SalePrice,
            8 => NativeDataAssetType::Phone,
            9 => NativeDataAssetType::Address,
            10 => NativeDataAssetType::Desc2,
            11 => NativeDataAssetType::DisplayUrl,
            12 => NativeDataAssetType::CtatextButtonText,
            _ => NativeDataAssetType::Unknown(value),
        }
    }
}

impl From<NativeDataAssetType> for i64 {
    fn from(value: NativeDataAssetType) -> Self {
        match value {
            NativeDataAssetType::Sponsored => 1,
            NativeDataAssetType::Desc => 2,
            NativeDataAssetType::Rating => 3,
            NativeDataAssetType::Likes => 4,
            NativeDataAssetType::Downloads => 5,
            NativeDataAssetType::Price => 6,
            NativeDataAssetType::SalePrice => 7,
            NativeDataAssetType::Phone => 8,
            NativeDataAssetType::Address => 9,
            NativeDataAssetType::Desc2 => 10,
            NativeDataAssetType::DisplayUrl => 11,
            NativeDataAssetType::CtatextButtonText => 12,
            NativeDataAssetType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(NativeDataAssetType);
