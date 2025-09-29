#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Creative Subtypes - Audio/Video
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum CreativeSubtypeAudioVideo {
    Vast1_0,
    Vast2_0,
    Vast3_0,
    VastWrapper1_0,
    VastWrapper2_0,
    VastWrapper3_0,
    Vast4_0,
    VastWrapper4_0,
    Daast1_0,
    DaastWrapper1_0,
    Vast4_1,
    VastWrapper4_1,
    Vast4_2,
    VastWrapper4_2,
    Vast4_3,
    VastWrapper4_3,
    Unknown(i64),
}

impl From<i64> for CreativeSubtypeAudioVideo {
    fn from(value: i64) -> Self {
        match value {
            1 => CreativeSubtypeAudioVideo::Vast1_0,
            2 => CreativeSubtypeAudioVideo::Vast2_0,
            3 => CreativeSubtypeAudioVideo::Vast3_0,
            4 => CreativeSubtypeAudioVideo::VastWrapper1_0,
            5 => CreativeSubtypeAudioVideo::VastWrapper2_0,
            6 => CreativeSubtypeAudioVideo::VastWrapper3_0,
            7 => CreativeSubtypeAudioVideo::Vast4_0,
            8 => CreativeSubtypeAudioVideo::VastWrapper4_0,
            9 => CreativeSubtypeAudioVideo::Daast1_0,
            10 => CreativeSubtypeAudioVideo::DaastWrapper1_0,
            11 => CreativeSubtypeAudioVideo::Vast4_1,
            12 => CreativeSubtypeAudioVideo::VastWrapper4_1,
            13 => CreativeSubtypeAudioVideo::Vast4_2,
            14 => CreativeSubtypeAudioVideo::VastWrapper4_2,
            15 => CreativeSubtypeAudioVideo::Vast4_3,
            16 => CreativeSubtypeAudioVideo::VastWrapper4_3,
            _ => CreativeSubtypeAudioVideo::Unknown(value),
        }
    }
}

impl From<CreativeSubtypeAudioVideo> for i64 {
    fn from(value: CreativeSubtypeAudioVideo) -> Self {
        match value {
            CreativeSubtypeAudioVideo::Vast1_0 => 1,
            CreativeSubtypeAudioVideo::Vast2_0 => 2,
            CreativeSubtypeAudioVideo::Vast3_0 => 3,
            CreativeSubtypeAudioVideo::VastWrapper1_0 => 4,
            CreativeSubtypeAudioVideo::VastWrapper2_0 => 5,
            CreativeSubtypeAudioVideo::VastWrapper3_0 => 6,
            CreativeSubtypeAudioVideo::Vast4_0 => 7,
            CreativeSubtypeAudioVideo::VastWrapper4_0 => 8,
            CreativeSubtypeAudioVideo::Daast1_0 => 9,
            CreativeSubtypeAudioVideo::DaastWrapper1_0 => 10,
            CreativeSubtypeAudioVideo::Vast4_1 => 11,
            CreativeSubtypeAudioVideo::VastWrapper4_1 => 12,
            CreativeSubtypeAudioVideo::Vast4_2 => 13,
            CreativeSubtypeAudioVideo::VastWrapper4_2 => 14,
            CreativeSubtypeAudioVideo::Vast4_3 => 15,
            CreativeSubtypeAudioVideo::VastWrapper4_3 => 16,
            CreativeSubtypeAudioVideo::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(CreativeSubtypeAudioVideo);
