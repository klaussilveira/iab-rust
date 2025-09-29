#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Volume Normalization Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum VolumeNormalizationMode {
    None,
    AdVolumeAverageNormalizeToContent,
    AdVolumePeakNormalizeToContent,
    AdLoudnessNormalizeToContent,
    Custom,
    Unknown(i64),
}

impl From<i64> for VolumeNormalizationMode {
    fn from(value: i64) -> Self {
        match value {
            0 => VolumeNormalizationMode::None,
            1 => VolumeNormalizationMode::AdVolumeAverageNormalizeToContent,
            2 => VolumeNormalizationMode::AdVolumePeakNormalizeToContent,
            3 => VolumeNormalizationMode::AdLoudnessNormalizeToContent,
            4 => VolumeNormalizationMode::Custom,
            _ => VolumeNormalizationMode::Unknown(value),
        }
    }
}

impl From<VolumeNormalizationMode> for i64 {
    fn from(value: VolumeNormalizationMode) -> Self {
        match value {
            VolumeNormalizationMode::None => 0,
            VolumeNormalizationMode::AdVolumeAverageNormalizeToContent => 1,
            VolumeNormalizationMode::AdVolumePeakNormalizeToContent => 2,
            VolumeNormalizationMode::AdLoudnessNormalizeToContent => 3,
            VolumeNormalizationMode::Custom => 4,
            VolumeNormalizationMode::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(VolumeNormalizationMode);
