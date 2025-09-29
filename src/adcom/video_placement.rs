use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::companion::Companion;
use super::enums::{
    PlacementSubtypeVideo, PlacementPosition, ClickType,
    ApiFramework, CreativeSubtypeAudioVideo, SizeUnit, DeliveryMethod,
    LinearityMode, PlaybackMethod, PlaybackCessationMode, PodSequence,
    SlotPositionInPod, CompanionType, ExpandableDirection
};
use crate::defaults::{default_i64_one, default_size_unit};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: VideoPlacement
/// This object signals that the placement may be a video placement and provides additional
/// detail about permitted video ads (e.g., VAST).
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct VideoPlacement {
    /// Placement subtype. Refer to List: Plcmt Subtypes - Video.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<PlacementSubtypeVideo>>"))]
    pub ptype: Option<PlacementSubtypeVideo>,

    /// Placement position on screen. Refer to List: Placement Positions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<PlacementPosition>>"))]
    pub pos: Option<PlacementPosition>,

    /// Indicates the start delay in seconds for pre-roll, mid-roll, or post-roll placements.
    /// For additional generic values, refer to List: Start Delay Modes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub delay: Option<i64>,

    /// Indicates if the placement imposes ad skippability, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub skip: Option<i64>,

    /// The placement allows creatives of total duration greater than this number of seconds
    /// to be skipped; only applicable if the ad is skippable.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub skipmin: i64,

    /// Number of seconds a creative must play before the placement enables skipping;
    /// only applicable if the ad is skippable.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub skipafter: i64,

    /// Playback method(s) in use for this placement. Refer to List: Playback Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<PlaybackMethod>>>"))]
    pub playmethod: Option<Vec<PlaybackMethod>>,

    /// The event that causes playback to end for this placement.
    /// Refer to List: Playback Cessation Modes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<PlaybackCessationMode>>"))]
    pub playend: Option<PlaybackCessationMode>,

    /// Indicates the click type of the placement. Refer to List: Click Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<ClickType>>"))]
    pub clktype: Option<ClickType>,

    /// Array of supported mime types (e.g., "video/mp4"). If omitted, all types are assumed.
    #[cfg_attr(feature="coercion", serde_as(as="Vec<AsString>"))]
    pub mime: Vec<String>,

    /// List of supported APIs for this placement. If an API is not explicitly listed,
    /// it is assumed to be unsupported. Refer to List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ApiFramework>>>"))]
    pub api: Option<Vec<ApiFramework>>,

    /// Creative subtypes permitted for this placement.
    /// Refer to List: Creative Subtypes - Audio/Video.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<CreativeSubtypeAudioVideo>>>"))]
    pub ctype: Option<Vec<CreativeSubtypeAudioVideo>>,

    /// Width of the placement in units specified by unit.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Height of the placement in units specified by unit.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// Units of size used for w and h attributes. Refer to List: Size Units.
    #[serde(default="default_size_unit")]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<SizeUnit>"))]
    pub unit: SizeUnit,

    /// Minimum creative duration in seconds. This field is mutually exclusive with rqddurs;
    /// only one of mindur and rqddurs may be in a bid request.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub mindur: Option<i64>,

    /// Maximum creative duration in seconds. This field is mutually exclusive with rqddurs;
    /// only one of maxdur and rqddurs may be in a bid request.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxdur: Option<i64>,

    /// Precise acceptable durations for video creatives in seconds. This field specifically
    /// targets the Live TV use case where non-exact ad durations would result in undesirable
    /// 'dead air'. This field is mutually exclusive with mindur and maxdur; if rqddurs is
    /// specified, mindur and maxdur must not be specified and vice versa.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsI64>>"))]
    pub rqddurs: Option<Vec<i64>>,

    /// Maximum extended creative duration if extension is allowed. If 0, extension is not
    /// allowed. If -1, extension is allowed and there is no time limit imposed. If greater
    /// than 0, then the value represents the number of seconds of extended play supported
    /// beyond the maxdur value.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub maxext: i64,

    /// Minimum bit rate of the creative in Kbps.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub minbitr: Option<i64>,

    /// Maximum bit rate of the creative in Kbps.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxbitr: Option<i64>,

    /// Array of supported creative delivery methods. If omitted, all can be assumed.
    /// Refer to List: Delivery Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<DeliveryMethod>>>"))]
    pub delivery: Option<Vec<DeliveryMethod>>,

    /// The maximum number of ads that can be played in an ad pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub maxseq: Option<i64>,

    /// Indicates the total amount of time in seconds that advertisers may fill for a
    /// "dynamic" video ad pod, or the dynamic portion of a "hybrid" ad pod. This field
    /// is required only for the dynamic portion(s) of video ad pods. This field refers
    /// to the length of the entire ad break, whereas mindur/maxdur/rqddurs are constraints
    /// relating to the slots that make up the pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub poddur: Option<i64>,

    /// Unique identifier indicating that an impression opportunity belongs to a video ad pod.
    /// If multiple impression opportunities within a bid request share the same podid, this
    /// indicates that those impression opportunities belong to the same video ad pod.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub podid: Option<String>,

    /// The sequence (position) of the video ad pod within a content stream.
    /// Refer to List: Pod Sequence for guidance on the use of this field.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<PodSequence>"))]
    pub podseq: PodSequence,

    /// For video ad pods, this value indicates that the seller can guarantee delivery
    /// against the indicated slot position in the pod. Refer to List: Slot Position in Pod
    /// for guidance on the use of this field.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<SlotPositionInPod>"))]
    pub slotinpod: SlotPositionInPod,

    /// Minimum CPM per second. This is a price floor for the "dynamic" portion of a video
    /// ad pod, relative to the duration of bids an advertiser may submit.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub mincpmpersec: Option<f64>,

    /// Indicates if the creative must be linear, nonlinear, etc. If none specified,
    /// no restrictions are assumed. Refer to List: Linearity Modes. Note that this field
    /// describes the expected VAST response and not whether a placement is in-stream,
    /// out-stream, etc. For that, see ptype.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<LinearityMode>>"))]
    pub linear: Option<LinearityMode>,

    /// Indicates if letterboxing of 4:3 creatives into a 16:9 window is allowed,
    /// where 0=no, 1=yes.
    #[serde(default="default_i64_one")]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub boxing: i64,

    /// Array of objects indicating that companion ads are available and providing the
    /// specifications thereof. Refer to Object: Companion.
    pub comp: Option<Vec<Companion>>,

    /// Supported companion ad types; recommended if companion ads are specified in comp.
    /// Refer to List: Companion Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<CompanionType>>>"))]
    pub comptype: Option<Vec<CompanionType>>,

    /// Directions in which the creative (video placement) is permitted to expand.
    /// Refer to List: Expandable Directions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ExpandableDirection>>>"))]
    pub expdir: Option<Vec<ExpandableDirection>>,

    /// Directions in which the creative (video overlay) is permitted to expand.
    /// This is primarily used for non-linear videos. Refer to List: Expandable Directions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ExpandableDirection>>>"))]
    pub overlayexpdir: Option<Vec<ExpandableDirection>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
