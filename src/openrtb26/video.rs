use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::{default_optional_i64_one, default_optional_i64_zero, default_optional_podseq_zero, default_optional_slotinpod_zero};
use crate::adcom;
use super::{Banner, DurFloors};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Represents a video impression.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Video {
    /// Content MIME types supported.
    #[cfg_attr(feature="coercion", serde_as(as="Vec<AsString>"))]
    pub mimes: Vec<String>,
    /// Minimum video ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_one")]
    pub minduration: Option<i64>,
    /// Maximum video ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxduration: Option<i64>,
    /// Start delay in seconds for placements. Refer to `AdCOM 1.0` List: Start Delay Modes. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub startdelay: Option<i64>,
    /// Maximum number of ads in a dynamic video ad pod. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxseq: Option<i64>,
    /// Total time in seconds for a dynamic video ad pod. Required for dynamic portions. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub poddur: Option<i64>,
    /// Array of supported video protocols. Refer to `AdCOM 1.0` List: Creative Subtypes - Audio/Video. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CreativeSubtypeAudioVideo>>>"))]
    pub protocols: Option<Vec<adcom::enums::CreativeSubtypeAudioVideo>>,
    /// Width of the video player in DIPS. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,
    /// Height of the video player in DIPS. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,
    /// Unique identifier for video ad pod impression belongs to.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub podid: Option<String>,
    /// Sequence (position) of video ad pod within content stream. Refer to `AdCOM 1.0` List: Pod Sequence.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::PodSequence>>"))]
    #[serde(default="default_optional_podseq_zero")]
    pub podseq: Option<adcom::enums::PodSequence>,
    /// Precise acceptable durations for video creatives in seconds. Mutually exclusive with minduration/maxduration.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsI64>>"))]
    pub rqddurs: Option<Vec<i64>>,
    /// DEPRECATED. Use plcmt instead.
    #[deprecated(since="2.6.2", note="Use plcmt instead")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub placement: Option<i64>,
    /// Video placement type for the impression. Refer to `AdCOM 1.0` List: Plcmt Subtypes - Video.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::PlacementSubtypeVideo>>"))]
    pub plcmt: Option<adcom::enums::PlacementSubtypeVideo>,
    /// Indicates linearity. Refer to `AdCOM 1.0` List: Linearity Modes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::LinearityMode>>"))]
    pub linearity: Option<adcom::enums::LinearityMode>,
    /// Indicates if player allows skipping (0=no, 1=yes).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub skip: Option<i64>,
    /// Min duration before skip is allowed (seconds); only applicable if skip=1.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub skipmin: Option<i64>,
    /// Seconds video must play before skipping is enabled; only applicable if skip=1.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub skipafter: Option<i64>,
    /// DEPRECATED as of `OpenRTB` 2.6. Use slotinpod.
    #[deprecated(since="2.6.0", note="Use slotinpod")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub sequence: Option<i64>,
    /// Seller guarantees delivery against indicated slot position in pod. Refer to `AdCOM 1.0` List: Slot Position in Pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::SlotPositionInPod>>"))]
    #[serde(default="default_optional_slotinpod_zero")]
    pub slotinpod: Option<adcom::enums::SlotPositionInPod>,
    /// Minimum CPM per second for dynamic portion of video ad pod.
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
    /// Indicates if letter-boxing is allowed (0=no, 1=yes).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_one")]
    pub boxingallowed: Option<i64>,
    /// Playback methods that may be in use. Refer to `AdCOM 1.0` List: Playback Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::PlaybackMethod>>>"))]
    pub playbackmethod: Option<Vec<adcom::enums::PlaybackMethod>>,
    /// Event causing playback end. Refer to `AdCOM 1.0` List: Playback Cessation Modes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::PlaybackCessationMode>>"))]
    pub playbackend: Option<adcom::enums::PlaybackCessationMode>,
    /// Supported delivery methods. Refer to `AdCOM 1.0` List: Delivery Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::DeliveryMethod>>>"))]
    pub delivery: Option<Vec<adcom::enums::DeliveryMethod>>,
    /// Ad position on screen. Refer to `AdCOM 1.0` List: Placement Positions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::PlacementPosition>>"))]
    pub pos: Option<adcom::enums::PlacementPosition>,
    /// Array of Banner objects if companion ads available.
    pub companionad: Option<Vec<Banner>>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::ApiFramework>>>"))]
    pub api: Option<Vec<adcom::enums::ApiFramework>>,
    /// Supported VAST companion ad types. Refer to `AdCOM 1.0` List: Companion Types. Recommended if companionad included.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::CompanionType>>>"))]
    pub companiontype: Option<Vec<adcom::enums::CompanionType>>,
    /// PROVISIONAL. Pod deduplication settings. Refer to `AdCOM 1.0` List: Pod Deduplication.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<adcom::enums::PodDeduplication>>>"))]
    pub poddedupe: Option<Vec<adcom::enums::PodDeduplication>>,
    /// Array of `DurFloors` objects indicating floor prices for various video durations.
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
