#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Creative Attributes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum CreativeAttribute {
    AudioAutoPlay,
    AudioUserInitiated,
    ExpandableAutomatic,
    ExpandableClickInitiated,
    ExpandableRolloverInitiated,
    VideoInBannerAutoPlay,
    VideoInBannerUserInitiated,
    Pop,
    ProvocativeOrSuggestive,
    ExtremeAnimation,
    Surveys,
    TextOnly,
    UserInteractive,
    AlertStyle,
    HasAudioOnOffButton,
    SkippableAdButton,
    AdobeFlash,
    Responsive,
    Unknown(i64),
}

impl From<i64> for CreativeAttribute {
    fn from(value: i64) -> Self {
        match value {
            1 => CreativeAttribute::AudioAutoPlay,
            2 => CreativeAttribute::AudioUserInitiated,
            3 => CreativeAttribute::ExpandableAutomatic,
            4 => CreativeAttribute::ExpandableClickInitiated,
            5 => CreativeAttribute::ExpandableRolloverInitiated,
            6 => CreativeAttribute::VideoInBannerAutoPlay,
            7 => CreativeAttribute::VideoInBannerUserInitiated,
            8 => CreativeAttribute::Pop,
            9 => CreativeAttribute::ProvocativeOrSuggestive,
            10 => CreativeAttribute::ExtremeAnimation,
            11 => CreativeAttribute::Surveys,
            12 => CreativeAttribute::TextOnly,
            13 => CreativeAttribute::UserInteractive,
            14 => CreativeAttribute::AlertStyle,
            15 => CreativeAttribute::HasAudioOnOffButton,
            16 => CreativeAttribute::SkippableAdButton,
            17 => CreativeAttribute::AdobeFlash,
            18 => CreativeAttribute::Responsive,
            _ => CreativeAttribute::Unknown(value),
        }
    }
}

impl From<CreativeAttribute> for i64 {
    fn from(value: CreativeAttribute) -> Self {
        match value {
            CreativeAttribute::AudioAutoPlay => 1,
            CreativeAttribute::AudioUserInitiated => 2,
            CreativeAttribute::ExpandableAutomatic => 3,
            CreativeAttribute::ExpandableClickInitiated => 4,
            CreativeAttribute::ExpandableRolloverInitiated => 5,
            CreativeAttribute::VideoInBannerAutoPlay => 6,
            CreativeAttribute::VideoInBannerUserInitiated => 7,
            CreativeAttribute::Pop => 8,
            CreativeAttribute::ProvocativeOrSuggestive => 9,
            CreativeAttribute::ExtremeAnimation => 10,
            CreativeAttribute::Surveys => 11,
            CreativeAttribute::TextOnly => 12,
            CreativeAttribute::UserInteractive => 13,
            CreativeAttribute::AlertStyle => 14,
            CreativeAttribute::HasAudioOnOffButton => 15,
            CreativeAttribute::SkippableAdButton => 16,
            CreativeAttribute::AdobeFlash => 17,
            CreativeAttribute::Responsive => 18,
            CreativeAttribute::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(CreativeAttribute);
