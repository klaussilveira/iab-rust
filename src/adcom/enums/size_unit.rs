/// List: Size Units
/// The following table lists the units of height and width used by creatives, assets,
/// and placement specifications where noted.
#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Size Units
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum SizeUnit {
    Unknown(i64),
    #[default]
    DeviceIndependentPixels,
    Inches,
    Centimeters,
}

impl From<i64> for SizeUnit {
    fn from(value: i64) -> Self {
        match value {
            0 => SizeUnit::Unknown(0),
            1 => SizeUnit::DeviceIndependentPixels,
            2 => SizeUnit::Inches,
            3 => SizeUnit::Centimeters,
            _ => SizeUnit::Unknown(value),
        }
    }
}

impl From<SizeUnit> for i64 {
    fn from(value: SizeUnit) -> Self {
        match value {
            SizeUnit::DeviceIndependentPixels => 1,
            SizeUnit::Inches => 2,
            SizeUnit::Centimeters => 3,
            SizeUnit::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(SizeUnit);
