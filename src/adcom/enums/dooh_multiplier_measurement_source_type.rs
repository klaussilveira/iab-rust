#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: DOOH Multiplier Measurement Source Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DoohMultiplierMeasurementSourceType {
    Unknown(i64),
    Measurement,
    Impression,
}

impl From<i64> for DoohMultiplierMeasurementSourceType {
    fn from(value: i64) -> Self {
        match value {
            0 => DoohMultiplierMeasurementSourceType::Unknown(0),
            1 => DoohMultiplierMeasurementSourceType::Measurement,
            2 => DoohMultiplierMeasurementSourceType::Impression,
            _ => DoohMultiplierMeasurementSourceType::Unknown(value),
        }
    }
}

impl From<DoohMultiplierMeasurementSourceType> for i64 {
    fn from(value: DoohMultiplierMeasurementSourceType) -> Self {
        match value {
            DoohMultiplierMeasurementSourceType::Measurement => 1,
            DoohMultiplierMeasurementSourceType::Impression => 2,
            DoohMultiplierMeasurementSourceType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DoohMultiplierMeasurementSourceType);
