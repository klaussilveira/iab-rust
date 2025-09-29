use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::{default_optional_i64_zero, default_optional_f64_zero, default_optional_string_usd};
use super::{Metric, Banner, Video, Audio, Native, Pmp, Qty, Refresh};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Imp
/// This object describes an ad placement or impression being auctioned. A single
/// bid request can include multiple `Imp` objects, a use case for which might be
/// an exchange that supports selling all ad positions on a given page. Each `Imp`
/// object has a required ID so that bids can reference them individually.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Imp {
    /// A unique identifier for this impression within the content of the bid
    /// request (typically, starts with 1 and increments).
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,
    /// An array of `Metric` object (section 3.2.5).
    pub metric: Option<Vec<Metric>>,
    /// A `Banner` object (section 3.2.6); required if this impression is
    /// offered as a banner ad opportunity.
    pub banner: Option<Banner>,
    /// A `Video` object (Section 3.2.7); required if this impression is
    /// offered as a video ad opportunity.
    pub video: Option<Video>,
    /// An `Audio` object (Section 3.2.8); required if this impression is
    /// offered as an audio ad opportunity.
    pub audio: Option<Audio>,
    /// A `Native` object (Section 3.2.9); required if this impression is
    /// offered as a native ad opportunity.
    #[serde(rename="native")]
    pub native_markup: Option<Native>,
    /// A `Pmp` object (Section 3.2.11) containing any private marketplace
    /// deals in effect for this impression.
    pub pmp: Option<Pmp>,
    /// Name of ad mediation partner, SDK technology, or player responsible
    /// for rendering ad (typically video or mobile). Used by some ad servers
    /// to customize ad code by partner. Recommended for video and/or apps.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub displaymanager: Option<String>,
    /// Version of ad mediation partner, SDK technology, or player responsible
    /// for rendering ad (typically video or mobile). Used by some ad servers
    /// to customize ad code by partner. Recommended for video and/or apps.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub displaymanagerver: Option<String>,
    /// 1=the ad is interstitial or full screen, 0=not interstitial.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub instl: Option<i64>,
    /// Identifier for specific ad placement or ad tag that was used to
    /// initiate the auction. This can be useful for debugging of any
    /// issues, or for optimization by the buyer.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub tagid: Option<String>,
    /// Minimum bid for this impression expressed in CPM.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    #[serde(default="default_optional_f64_zero")]
    pub bidfloor: Option<f64>,
    /// Currency specified using ISO-4217 alpha codes. This may be different
    /// from bid currency returned by bidder if this is allowed by the
    /// exchange. This currency sets the default for all floors specified
    /// in the `Imp` object.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    #[serde(default="default_optional_string_usd")]
    pub bidfloorcur: Option<String>,
    /// Indicates the type of browser opened upon clicking the creative in
    /// an app, where 0=embedded, 1=native. Note that the Safari View
    /// Controller in iOS 9.x devices is considered a native browser for
    /// purposes of this attribute.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub clickbrowser: Option<i64>,
    /// Flag to indicate if the impression requires secure HTTPS URL creative
    /// assets and markup, where 0=non-secure, 1=secure. If omitted, the
    /// secure state is unknown, but non-secure HTTP support can be assumed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub secure: Option<i64>,
    /// Array of exchange-specific names of supported iframe busters.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub iframebuster: Option<Vec<String>>,
    /// Indicates whether the user receives a reward for viewing the ad,
    /// where 0=no, 1=yes. Typically video ad implementations allow
    /// users to read an additional news article for free, receive an extra
    /// life in a game, or get a sponsored ad-free music session. The reward
    /// is typically distributed after the video ad is completed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub rwdd: Option<i64>,
    /// Indicates if server-side ad insertion (e.g., stitching an ad into an
    /// audio or video stream) is in use and the impact of this on asset and
    /// tracker retrieval, where 0=status unknown, 1=all client-side
    /// (i.e., not server-side), 2=assets stitched server-side but tracking
    /// pixels fired client-side, 3=all server-side.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub ssai: Option<i64>,
    /// Advisory as to the number of seconds that may elapse between the
    /// auction and the actual impression.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub exp: Option<i64>,
    /// A means of passing a multiplier in the bid request, representing
    /// the total quantity of impressions for adverts that display to more
    /// than one person.
    pub qty: Option<Qty>,
    /// Timestamp when the item is estimated to be fulfilled (e.g. when a
    /// DOOH impression will be displayed) in Unix format
    /// (i.e., milliseconds since the epoch).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub dt: Option<f64>,
    /// Details about ad slots being refreshed automatically. (Section 3.2.33)
    pub refresh: Option<Refresh>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
