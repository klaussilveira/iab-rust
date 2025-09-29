use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::{default_optional_i64_zero, default_optional_podseq_zero, default_optional_slotinpod_zero};
use crate::adcom;
use super::{Banner, DurFloors};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Represents an audio type impression.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Audio {
    /// Content MIME types supported.
    #[cfg_attr(feature="coercion", serde_as(as="Vec<AsString>"))]
    pub mimes: Vec<String>,
    /// Minimum audio ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub minduration: Option<i64>,
    /// Maximum audio ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxduration: Option<i64>,
    /// Total time in seconds for a dynamic audio ad pod. Required for dynamic portions. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub poddur: Option<i64>,
    /// Array of supported audio protocols. Refer to `AdCOM 1.0` List: Creative Subtypes - Audio/Video. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CreativeSubtypeAudioVideo>>>"))]
    pub protocols: Option<Vec<adcom::enums::CreativeSubtypeAudioVideo>>,
    /// Start delay in seconds for placements. Refer to `AdCOM 1.0` List: Start Delay Modes. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::StartDelayMode>>"))]
    pub startdelay: Option<adcom::enums::StartDelayMode>,
    /// Precise acceptable durations for audio creatives in seconds. Mutually exclusive with minduration/maxduration.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsI64>>"))]
    pub rqddurs: Option<Vec<i64>>,
    /// Unique identifier for audio ad pod impression belongs to.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub podid: Option<String>,
    /// Sequence (position) of audio ad pod within content stream. Refer to `AdCOM 1.0` List: Pod Sequence.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::PodSequence>>"))]
    #[serde(default="default_optional_podseq_zero")]
    pub podseq: Option<adcom::enums::PodSequence>,
    /// DEPRECATED as of `OpenRTB` 2.6. Use slotinpod.
    #[deprecated(since="2.6.0", note="Use slotinpod")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub sequence: Option<i64>,
    /// Seller guarantees delivery against indicated slot position in pod. Refer to `AdCOM 1.0` List: Slot Position in Pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::SlotPositionInPod>>"))]
    #[serde(default="default_optional_slotinpod_zero")]
    pub slotinpod: Option<adcom::enums::SlotPositionInPod>,
    /// Minimum CPM per second for dynamic portion of audio ad pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub mincpmpersec: Option<f64>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CreativeAttribute>>>"))]
    pub battr: Option<Vec<adcom::enums::CreativeAttribute>>,
    /// Maximum extended ad duration if allowed. If blank/0, not allowed. -1=no limit. >0=seconds of extension.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxextended: Option<i64>,
    /// Minimum bit rate in Kbps.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub minbitrate: Option<i64>,
    /// Maximum bit rate in Kbps.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxbitrate: Option<i64>,
    /// Supported delivery methods. Refer to `AdCOM 1.0` List: Delivery Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::DeliveryMethod>>>"))]
    pub delivery: Option<Vec<adcom::enums::DeliveryMethod>>,
    /// Array of Banner objects if companion ads available.
    pub companionad: Option<Vec<Banner>>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::ApiFramework>>>"))]
    pub api: Option<Vec<adcom::enums::ApiFramework>>,
    /// Supported companion ad types. Refer to `AdCOM 1.0` List: Companion Types. Recommended if companionad included.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CompanionType>>>"))]
    pub companiontype: Option<Vec<adcom::enums::CompanionType>>,
    /// Maximum number of ads that can be played in an ad pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxseq: Option<i64>,
    /// Type of audio feed. Refer to `AdCOM 1.0` List: Feed Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::FeedType>>"))]
    pub feed: Option<adcom::enums::FeedType>,
    /// Indicates if ad is stitched (0=no, 1=yes).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub stitched: Option<i64>,
    /// Volume normalization mode. Refer to `AdCOM 1.0` List: Volume Normalization Modes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::VolumeNormalizationMode>>"))]
    pub nvol: Option<adcom::enums::VolumeNormalizationMode>,
    /// Array of `DurFloors` objects indicating floor prices for various audio durations.
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
