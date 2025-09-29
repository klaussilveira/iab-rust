#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Playback Methods
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum PlaybackMethod {
    PageLoadSoundOn,
    PageLoadSoundOff,
    ClickSoundOn,
    MouseOverSoundOn,
    ViewportSoundOn,
    ViewportSoundOffMuted,
    Unknown(i64),
}

impl From<i64> for PlaybackMethod {
    fn from(value: i64) -> Self {
        match value {
            1 => PlaybackMethod::PageLoadSoundOn,
            2 => PlaybackMethod::PageLoadSoundOff,
            3 => PlaybackMethod::ClickSoundOn,
            4 => PlaybackMethod::MouseOverSoundOn,
            5 => PlaybackMethod::ViewportSoundOn,
            6 => PlaybackMethod::ViewportSoundOffMuted,
            _ => PlaybackMethod::Unknown(value),
        }
    }
}

impl From<PlaybackMethod> for i64 {
    fn from(value: PlaybackMethod) -> Self {
        match value {
            PlaybackMethod::PageLoadSoundOn => 1,
            PlaybackMethod::PageLoadSoundOff => 2,
            PlaybackMethod::ClickSoundOn => 3,
            PlaybackMethod::MouseOverSoundOn => 4,
            PlaybackMethod::ViewportSoundOn => 5,
            PlaybackMethod::ViewportSoundOffMuted => 6,
            PlaybackMethod::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(PlaybackMethod);
