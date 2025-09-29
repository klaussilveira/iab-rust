#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Playback Cessation Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum PlaybackCessationMode {
    OnVideoCompletion,
    OnLeavingViewport,
    OnLeavingFloatsViewport,
    Unknown(i64),
}

impl From<i64> for PlaybackCessationMode {
    fn from(value: i64) -> Self {
        match value {
            1 => PlaybackCessationMode::OnVideoCompletion,
            2 => PlaybackCessationMode::OnLeavingViewport,
            3 => PlaybackCessationMode::OnLeavingFloatsViewport,
            _ => PlaybackCessationMode::Unknown(value),
        }
    }
}

impl From<PlaybackCessationMode> for i64 {
    fn from(value: PlaybackCessationMode) -> Self {
        match value {
            PlaybackCessationMode::OnVideoCompletion => 1,
            PlaybackCessationMode::OnLeavingViewport => 2,
            PlaybackCessationMode::OnLeavingFloatsViewport => 3,
            PlaybackCessationMode::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(PlaybackCessationMode);
