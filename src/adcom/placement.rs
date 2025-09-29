use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{display_placement::DisplayPlacement, video_placement::VideoPlacement, audio_placement::AudioPlacement};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Placement
/// This object represents the properties of a placement and the characteristics of ads
/// permitted to be rendering within them. Placements of all types begin with this object
/// as their root. It contains one or more subtype objects (i.e., display, video, audio)
/// that define the kinds of media permitted as well as media specific placement behaviors.
///
/// The other attributes in this object apply to all aspects and substructures of the
/// placement (i.e., the same language, secure status, etc. apply to all media types
/// and native assets as applicable).
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Placement {
    /// Identifier for specific ad placement or ad tag; unique within a distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub tagid: Option<String>,

    /// Indicates if server-side ad insertion (e.g., stitching an ad into an audio or video
    /// stream) is in use and the impact of this on asset and tracker retrieval, where
    /// 0=status unknown, 1=all client-side (i.e., not server-side), 2=assets stitched
    /// server-side but tracking pixels fired client-side, 3=all server-side.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub ssai: i64,

    /// Name of ad mediation partner, SDK technology, or player responsible for rendering
    /// ad (typically video, audio, or mobile); used by some ad servers to customize ad
    /// code by partner.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub sdk: Option<String>,

    /// Version of the SDK specified in the sdk attribute.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub sdkver: Option<String>,

    /// Indicates whether the user receives a reward for viewing the ad, where 0=no, 1=yes.
    /// Typically video ad implementations allow users to read an additional news article for
    /// free, receive an extra life in a game, or get a sponsored ad-free music session.
    /// The reward is typically distributed after the video ad is completed.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub reward: i64,

    /// Allow list of permitted languages of the creative using ISO-639-1-alpha-2. In practice,
    /// vendors using this object may elect an alternate standard (e.g., BCP-47) in which case
    /// this must be communicated beforehand. Omission of this attribute indicates there are
    /// no restrictions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub wlang: Option<Vec<String>>,

    /// Flag to indicate if the creative is secure (i.e., uses HTTPS for all assets and markup),
    /// where 0=no, 1=yes. There is no default and thus if omitted, the secure state is
    /// unknown. However, as a practical matter, the safe assumption is to treat unknown as non-secure.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub secure: Option<i64>,

    /// Indicates if including markup is supported (i.e., the various adm attributes throughout
    /// the Placement structure), where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub admx: Option<i64>,

    /// Indicates if retrieving markup via URL reference is supported (i.e., the various curl
    /// attributes throughout the Placement structure), where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub curlx: Option<i64>,

    /// Placement Subtype Object that indicates that this may be a display placement and
    /// provides additional detail related thereto. Refer to Object: DisplayPlacement.
    /// * At least one placement subtype object is required.
    pub display: Option<DisplayPlacement>,

    /// Placement Subtype Object that indicates that this may be a video placement and
    /// provides additional detail related thereto. Refer to Object: VideoPlacement.
    /// * At least one placement subtype object is required.
    pub video: Option<VideoPlacement>,

    /// Placement Subtype Object that indicates that this may be an audio placement and
    /// provides additional detail related thereto. Refer to Object: AudioPlacement.
    /// * At least one placement subtype object is required.
    pub audio: Option<AudioPlacement>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
