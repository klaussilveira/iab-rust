use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Top-level enum to represent either a `BidRequest` or a `BidResponse`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenRtb {
    BidRequest(BidRequest),
    BidResponse(BidResponse),
}

/// Top-level bid request object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BidRequest {
    /// ID of the bid request, assigned by the exchange.
    pub id: String,
    /// Array of Imp objects representing the impressions offered. At least 1 Imp object is required.
    pub imp: Vec<Imp>,
    /// Details via a Site object about the publisher's website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
    /// Details via an App object about the publisher's app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
    /// Details via a DOOH object about the publisher's digital out-of-home inventory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dooh: Option<DOOH>,
    /// Details via a Device object about the user's device. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// Details via a User object about the human user of the device. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Indicator of test mode (0 = live, 1 = test).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub test: Option<i64>,
    /// Auction type (1 = First Price, 2 = Second Price Plus).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub at: Option<i64>,
    /// Maximum time in milliseconds for bids to be received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<i64>,
    /// Allowed list of buyer seats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,
    /// Block list of buyer seats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bseat: Option<Vec<String>>,
    /// Flag to indicate if impressions offered represent all available (0 = no/unknown, 1 = yes).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub allimps: Option<i64>,
    /// Array of allowed currencies for bids (ISO-4217 alpha codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<Vec<String>>,
    /// Allowed list of languages for creatives (ISO-639-1-alpha-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<String>>,
    /// Allowed list of languages for creatives (IETF BCP 47).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlangb: Option<Vec<String>>,
    /// Allowed advertiser categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acat: Option<Vec<String>>,
    /// Blocked advertiser categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,
    /// Taxonomy in use for bcat/acat. Refer to `AdCOM 1.0` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub cattax: Option<i64>,
    /// Block list of advertisers by their domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badv: Option<Vec<String>>,
    /// Block list of applications by their app store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bapp: Option<Vec<String>>,
    /// A Source object providing data about the inventory source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// A Regs object specifying regulations in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regs: Option<Regs>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Describes the source of the bid request upstream from the exchange.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// Entity responsible for final sale decision (0 = exchange, 1 = upstream source). Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fd: Option<i64>,
    /// Transaction ID common across participants. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    /// Payment ID chain string. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pchain: Option<String>,
    /// `SupplyChain` object. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schain: Option<SupplyChain>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Contains legal, governmental, or industry regulations applicable to the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Regs {
    /// Flag indicating if request is subject to COPPA (0 = no, 1 = yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coppa: Option<i64>,
    /// Flag indicating if request is subject to GDPR (0 = No, 1 = Yes, omission indicates unknown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<i64>,
    /// US Privacy String specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_privacy: Option<String>,
    /// Global Privacy Platform's consent string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpp: Option<String>,
    /// Array of GPP section IDs to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpp_sid: Option<Vec<i64>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Describes an ad placement or impression being auctioned.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Imp {
    /// Unique identifier for this impression within the bid request.
    pub id: String,
    /// Array of Metric objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<Vec<Metric>>,
    /// A Banner object; required if banner ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Banner>,
    /// A Video object; required if video ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// An Audio object; required if audio ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// A Native object; required if native ad opportunity.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "native")]
    pub native_markup: Option<Native>, // Renamed to avoid keyword conflict
    /// A Pmp object containing private marketplace deals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmp: Option<Pmp>,
    /// Name of ad mediation partner, SDK technology, or player. Recommended for video/apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaymanager: Option<String>,
    /// Version of ad mediation partner, SDK technology, or player. Recommended for video/apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaymanagerver: Option<String>,
    /// 1 = interstitial or full screen, 0 = not interstitial.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub instl: Option<i64>,
    /// Identifier for specific ad placement or ad tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagid: Option<String>,
    /// Minimum bid for this impression expressed in CPM.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<f64>
    pub bidfloor: Option<f64>,
    /// Currency for bidfloor (ISO-4217 alpha codes).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<String>
    pub bidfloorcur: Option<String>,
    /// Type of browser opened upon clicking the creative in an app (0 = embedded, 1 = native).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickbrowser: Option<i64>,
    /// Flag for secure HTTPS requirement (0 = non-secure, 1 = secure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i64>,
    /// Array of supported iframe busters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframebuster: Option<Vec<String>>,
    /// Indicates if user receives reward for viewing ad (0 = no, 1 = yes).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub rwdd: Option<i64>,
    /// Indicates server-side ad insertion usage (0=unknown, 1=client, 2=assets server/tracking client, 3=all server).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub ssai: Option<i64>,
    /// Advisory as to the number of seconds between auction and impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    /// Quantity multiplier object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Qty>,
    /// Timestamp when item is estimated to be fulfilled (Unix ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<f64>, // Using f64 as it can be float according to schema
    /// Details about ad slots being refreshed automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<Refresh>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Offers insight into the impression, like viewability or CTR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    /// Type of metric being presented.
    #[serde(rename = "type")]
    pub type_: String, // Renamed to avoid keyword conflict
    /// Value of the metric (probabilities 0.0–1.0).
    pub value: f64,
    /// Source of the value. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents a banner impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Banner {
    /// Array of Format objects representing permitted banner sizes. Recommended if h/w not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Vec<Format>>,
    /// Exact width in DIPS. Recommended if no Format objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
    /// Exact height in DIPS. Recommended if no Format objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Blocked banner ad types (1=XHTML Text, 2=XHTML Banner, 3=JavaScript, 4=iframe).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<Vec<i64>>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i64>>,
    /// Ad position on screen. Refer to `AdCOM 1.0` List: Placement Positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i64>,
    /// Content MIME types supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,
    /// Indicates if banner is in top frame (0 = no, 1 = yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topframe: Option<i64>,
    /// Directions banner may expand. Refer to `AdCOM 1.0` List: Expandable Directions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<i64>>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i64>>,
    /// Unique identifier for this banner object. Recommended for companion ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Companion banner rendering mode (0 = concurrent, 1 = end-card). Relevant for companion ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcm: Option<i64>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents a video impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Video {
    /// Content MIME types supported.
    pub mimes: Vec<String>,
    /// Minimum video ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub minduration: Option<i64>,
    /// Maximum video ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i64>,
    /// Start delay in seconds for placements. Refer to `AdCOM 1.0` List: Start Delay Modes. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i64>,
    /// Maximum number of ads in a dynamic video ad pod. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i64>,
    /// Total time in seconds for a dynamic video ad pod. Required for dynamic portions. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poddur: Option<i64>,
    /// Array of supported video protocols. Refer to `AdCOM 1.0` List: Creative Subtypes - Audio/Video. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i64>>,
    /// Width of the video player in DIPS. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
    /// Height of the video player in DIPS. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Unique identifier for video ad pod impression belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podid: Option<String>,
    /// Sequence (position) of video ad pod within content stream. Refer to `AdCOM 1.0` List: Pod Sequence.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub podseq: Option<i64>,
    /// Precise acceptable durations for video creatives in seconds. Mutually exclusive with minduration/maxduration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rqddurs: Option<Vec<i64>>,
    /// DEPRECATED. Use plcmt instead.
    #[deprecated(since = "2.6.2", note = "Use plcmt instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<i64>,
    /// Video placement type for the impression. Refer to `AdCOM 1.0` List: Plcmt Subtypes - Video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plcmt: Option<i64>,
    /// Indicates linearity. Refer to `AdCOM 1.0` List: Linearity Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearity: Option<i64>,
    /// Indicates if player allows skipping (0 = no, 1 = yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Min duration before skip is allowed (seconds); only applicable if skip=1.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub skipmin: Option<i64>,
    /// Seconds video must play before skipping is enabled; only applicable if skip=1.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub skipafter: Option<i64>,
    /// DEPRECATED as of `OpenRTB` 2.6. Use slotinpod.
    #[deprecated(since = "2.6.0", note = "Use slotinpod")]
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub sequence: Option<i64>,
    /// Seller guarantees delivery against indicated slot position in pod. Refer to `AdCOM 1.0` List: Slot Position in Pod.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub slotinpod: Option<i64>,
    /// Minimum CPM per second for dynamic portion of video ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mincpmpersec: Option<f64>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i64>>,
    /// Maximum extended ad duration if allowed. If blank/0, not allowed. -1=no limit. >0=seconds of extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxextended: Option<i64>,
    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i64>,
    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i64>,
    /// Indicates if letter-boxing is allowed (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub boxingallowed: Option<i64>,
    /// Playback methods that may be in use. Refer to `AdCOM 1.0` List: Playback Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbackmethod: Option<Vec<i64>>,
    /// Event causing playback end. Refer to `AdCOM 1.0` List: Playback Cessation Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbackend: Option<i64>,
    /// Supported delivery methods. Refer to `AdCOM 1.0` List: Delivery Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<i64>>,
    /// Ad position on screen. Refer to `AdCOM 1.0` List: Placement Positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i64>,
    /// Array of Banner objects if companion ads available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<Banner>>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i64>>,
    /// Supported VAST companion ad types. Refer to `AdCOM 1.0` List: Companion Types. Recommended if companionad included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<i64>>,
    /// PROVISIONAL. Pod deduplication settings. Refer to `AdCOM 1.0` List: Pod Deduplication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poddedupe: Option<Vec<i64>>,
    /// Array of `DurFloors` objects indicating floor prices for various video durations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents an audio type impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Audio {
    /// Content MIME types supported.
    pub mimes: Vec<String>,
    /// Minimum audio ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub minduration: Option<i64>,
    /// Maximum audio ad duration in seconds. Recommended. Mutually exclusive with rqddurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i64>,
    /// Total time in seconds for a dynamic audio ad pod. Required for dynamic portions. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poddur: Option<i64>,
    /// Array of supported audio protocols. Refer to `AdCOM 1.0` List: Creative Subtypes - Audio/Video. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i64>>,
    /// Start delay in seconds for placements. Refer to `AdCOM 1.0` List: Start Delay Modes. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<i64>,
    /// Precise acceptable durations for audio creatives in seconds. Mutually exclusive with minduration/maxduration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rqddurs: Option<Vec<i64>>,
    /// Unique identifier for audio ad pod impression belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podid: Option<String>,
    /// Sequence (position) of audio ad pod within content stream. Refer to `AdCOM 1.0` List: Pod Sequence.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub podseq: Option<i64>,
    /// DEPRECATED as of `OpenRTB` 2.6. Use slotinpod.
    #[deprecated(since = "2.6.0", note = "Use slotinpod")]
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub sequence: Option<i64>,
    /// Seller guarantees delivery against indicated slot position in pod. Refer to `AdCOM 1.0` List: Slot Position in Pod.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub slotinpod: Option<i64>,
    /// Minimum CPM per second for dynamic portion of audio ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mincpmpersec: Option<f64>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i64>>,
    /// Maximum extended ad duration if allowed. If blank/0, not allowed. -1=no limit. >0=seconds of extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxextended: Option<i64>,
    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i64>,
    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i64>,
    /// Supported delivery methods. Refer to `AdCOM 1.0` List: Delivery Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<i64>>,
    /// Array of Banner objects if companion ads available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<Banner>>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i64>>,
    /// Supported companion ad types. Refer to `AdCOM 1.0` List: Companion Types. Recommended if companionad included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<i64>>,
    /// Maximum number of ads that can be played in an ad pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i64>,
    /// Type of audio feed. Refer to `AdCOM 1.0` List: Feed Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<i64>,
    /// Indicates if ad is stitched (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stitched: Option<i64>,
    /// Volume normalization mode. Refer to `AdCOM 1.0` List: Volume Normalization Modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvol: Option<i64>,
    /// Array of `DurFloors` objects indicating floor prices for various audio durations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents a native type impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Native {
    /// Request payload complying with Native Ad Specification (JSON encoded string).
    pub request: String,
    /// Version of the Dynamic Native Ads API. Highly recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    /// List of supported API frameworks. Refer to `AdCOM 1.0` List: API Frameworks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i64>>,
    /// Blocked creative attributes. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i64>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents an allowed size (height/width) or Flex Ad parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    /// Width in DIPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
    /// Height in DIPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Relative width for ratio size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i64>,
    /// Relative height for ratio size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i64>,
    /// Minimum width in DIPS for ratio size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i64>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Private marketplace container for direct deals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Pmp {
    /// Indicator of auction eligibility (0=all bids, 1=restricted to deals).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub private_auction: Option<i64>,
    /// Array of Deal objects applicable to this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<Deal>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Constitutes a specific deal struck between a buyer and a seller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Deal {
    /// Unique identifier for the direct deal.
    pub id: String,
    /// Minimum bid for this impression expressed in CPM.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub bidfloor: Option<f64>,
    /// Currency for bidfloor (ISO-4217 alpha codes).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub bidfloorcur: Option<String>,
    /// Optional override of overall auction type (1=First Price, 2=Second Price Plus, 3=Deal Price).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<i64>,
    /// Allowed list of buyer seats for this deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,
    /// Array of advertiser domains allowed for this deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wadomain: Option<Vec<String>>,
    /// Indicates if deal is 'guaranteed' (0 = not guaranteed, 1 = guaranteed).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub guar: Option<i64>,
    /// Minimum CPM per second for video/audio opportunities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mincpmpersec: Option<f64>,
    /// Container for floor price by duration information (video/audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Details of the website calling for the impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Site {
    /// Exchange-specific site ID. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Site name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Domain of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Taxonomy in use for categories. Refer to `AdCOM` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub cattax: Option<i64>,
    /// Array of IAB Tech Lab content categories of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    /// Array of IAB Tech Lab content categories for the current site section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,
    /// Array of IAB Tech Lab content categories for the current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,
    /// URL of the page where the impression will be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Referrer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub refer: Option<String>, // Renamed to avoid keyword conflict
    /// Search string that caused navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Indicates if site optimized for mobile (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i64>,
    /// Indicates if site has privacy policy (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i64>,
    /// Details about the Publisher of the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,
    /// Details about the Content within the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    /// Comma separated list of keywords about the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// Array of keywords about the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,
    /// Domain for inventory authorization (ads.txt inventorypartnerdomain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventorypartnerdomain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Details of the non-browser application calling for the impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct App {
    /// Exchange-specific app ID. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// App name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Store ID of the app (bundle/package name or numeric ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,
    /// Domain of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// App store URL for an installed app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,
    /// Taxonomy in use for categories. Refer to `AdCOM` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub cattax: Option<i64>,
    /// Array of IAB Tech Lab content categories of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    /// Array of IAB Tech Lab content categories for the current app section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,
    /// Array of IAB Tech Lab content categories for the current page/view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,
    /// Application version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    /// Indicates if app has privacy policy (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i64>,
    /// 0 = app is free, 1 = paid version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i64>,
    /// Details about the Publisher of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,
    /// Details about the Content within the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    /// Comma separated list of keywords about the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// Array of keywords about the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,
    /// Domain for inventory authorization (app-ads.txt inventorypartnerdomain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventorypartnerdomain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Entity who directly supplies inventory to and is paid by the exchange.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Publisher {
    /// Exchange-specific seller ID (corresponds to `seller_id` in sellers.json).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Seller name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Taxonomy in use for categories. Refer to `AdCOM` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub cattax: Option<i64>,
    /// Array of IAB Tech Lab content categories of the publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    /// Highest level domain of the seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Describes the content in which the impression appears.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Content {
    /// ID uniquely identifying the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Episode number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i64>,
    /// Content title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Content series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    /// Content season.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    /// Artist credited with the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// Genre describing the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Taxonomy used for 'genres'. Default 9 (Content Cat Tax 3.1).
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub gtax: Option<i64>,
    /// Array of categories describing content genre. Taxonomy defined by gtax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<i64>>,
    /// Album the content belongs to (typically audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    /// International Standard Recording Code (ISO-3901).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,
    /// Details about the content Producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<Producer>,
    /// URL of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Taxonomy in use for 'cat'. Refer to `AdCOM` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub cattax: Option<i64>,
    /// Array of IAB Tech Lab content categories describing the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    /// Production quality. Refer to `AdCOM 1.0` List: Production Qualities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prodq: Option<i64>,
    /// Type of content. Refer to `AdCOM 1.0` List: Content Contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i64>,
    /// Content rating (e.g., MPAA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<String>,
    /// User rating of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userrating: Option<String>,
    /// Media rating per IQG guidelines. Refer to `AdCOM 1.0` List: Media Ratings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i64>,
    /// Comma separated list of keywords describing the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// Array of keywords describing the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,
    /// 0 = not live, 1 = content is live.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livestream: Option<i64>,
    /// 0 = indirect, 1 = direct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcerelationship: Option<i64>,
    /// Length of content in seconds (video/audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i64>,
    /// Content language using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Content language using IETF BCP 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langb: Option<String>,
    /// Indicator if content is embeddable (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddable: Option<i64>,
    /// Additional content data via Data objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,
    /// Details about the network the content is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// Details about the channel the content is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Defines the producer of the content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Producer {
    /// Content producer or originator ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Content producer or originator name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Taxonomy in use for categories. Refer to `AdCOM` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type to Option<i64>
    pub cattax: Option<i64>,
    /// Array of IAB Tech Lab content categories describing the producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    /// Highest level domain of the content producer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Information pertaining to the device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// Location of the device defined by a Geo object. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    /// Standard 'Do Not Track' flag (0=unrestricted, 1=do not track). Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnt: Option<i64>,
    /// 'Limit Ad Tracking' signal (0=unrestricted, 1=limited). Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i64>,
    /// Browser user agent string (raw).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<String>,
    /// Structured user agent information. Use instead of ua if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sua: Option<UserAgent>,
    /// IPv4 address closest to device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>, // Could use IpAddr type but string is safer for compatibility
    /// IPv6 address closest to device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>, // Could use IpAddr type but string is safer for compatibility
    /// General type of device. Refer to `AdCOM 1.0` List: Device Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<i64>,
    /// Device make.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// Device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Device operating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Device operating system version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,
    /// Hardware version of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,
    /// Physical height of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Physical width of the screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
    /// Screen size as pixels per linear inch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i64>,
    /// Ratio of physical pixels to device independent pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,
    /// Support for JavaScript (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<i64>,
    /// Geolocation API availability to banner JavaScript (0=no, 1=yes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geofetch: Option<i64>,
    /// Version of Flash supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashver: Option<String>,
    /// Browser language using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Browser language using IETF BCP 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langb: Option<String>,
    /// Carrier or ISP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// Mobile carrier as concatenated MCC-MNC code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,
    /// Network connection type. Refer to `AdCOM 1.0` List: Connection Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectiontype: Option<i64>,
    /// ID sanctioned for advertiser use (e.g., IDFA, AAID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifa: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didsha1: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didmd5: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidsha1: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidmd5: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macsha1: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Encapsulates geographic location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geo {
    /// Latitude (-90.0 to +90.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    /// Longitude (-180.0 to +180.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    /// Source of location data. Refer to `AdCOM 1.0` List: Location Types. Recommended.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>, // Renamed to avoid keyword conflict
    /// Estimated location accuracy in meters. Recommended for type=1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<i64>,
    /// Seconds since geolocation fix was established.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastfix: Option<i64>,
    /// Service used for IP address geolocation (type=2). Refer to `AdCOM 1.0` List: IP Location Services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipservice: Option<i64>,
    /// Country code using ISO-3166-1-alpha-3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Region code using ISO-3166-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Region code using FIPS 10-4 (withdrawn).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regionfips104: Option<String>,
    /// Google metro code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metro: Option<String>,
    /// City using UN Code for Trade & Transport Locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Local time as +/- minutes from UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utcoffset: Option<i64>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Information about the human user of the device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    /// Exchange-specific ID for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Buyer-specific ID for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<String>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<i64>,
    /// Deprecated as of `OpenRTB` 2.6.
    #[deprecated(since = "2.6.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Comma separated list of keywords, interests, or intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// Array of keywords about the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwarray: Option<Vec<String>>,
    /// Optional bidder data set in exchange's cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,
    /// Location of the user's home base (not necessarily current location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    /// Additional user data from third-party providers via Data objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Data>>,
    /// TCF Consent String when GDPR applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<String>,
    /// Extended identifiers support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eids: Option<Vec<EID>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Container for specifying additional data about a related object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// Exchange-specific ID for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Exchange-specific name for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of Segment objects containing actual data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<Segment>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Key-value pairs conveying specific units of data within a Data object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    /// ID of the data segment specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the data segment specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// String representation of the data segment value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Describes the network an ad will be displayed on.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    /// Unique identifier assigned by the publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Primary domain of the network. Recommend TLD+1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Describes the channel an ad will be displayed on.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// Unique identifier assigned by the publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Channel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Primary domain of the channel. Recommend TLD+1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents the chain of entities involved in the direct flow of payment for inventory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplyChain {
    /// Flag indicating if chain contains all nodes back to owner (0=no, 1=yes).
    pub complete: i64,
    /// Array of `SupplyChainNode` objects in order.
    pub nodes: Vec<SupplyChainNode>,
    /// Version of the supply chain specification.
    pub ver: String,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Defines the identity of an entity participating in the supply chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplyChainNode {
    /// Canonical domain name of advertising system.
    pub asi: String,
    /// Seller/reseller account ID within the advertising system.
    pub sid: String,
    /// `OpenRTB` `RequestId` issued by this seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    /// Name of the company paid for inventory under sid. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Business domain name of the entity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Indicates if node involved in payment flow (1=yes, 0=no). Should be 1 for v1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<i64>,
    /// Placeholder for advertising-system specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Extended Identifiers support. Contains UIDs from a single source/provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EID {
    /// Canonical domain name of entity that added the ID array element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserter: Option<String>,
    /// Canonical domain of the ID source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Technology providing the match method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
    /// Match method used by the matcher. Refer to `AdCOM 1.0` List: ID Match Methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm: Option<i64>,
    /// Array of extended ID UID objects from the given source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uids: Option<Vec<UID>>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// A single user identifier provided as part of extended identifiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UID {
    /// The identifier for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of user agent the ID is from. Highly recommended. Refer to `AdCOM 1.0` List: Agent Types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atype: Option<i64>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Structured user agent information based on User-Agent Client Hints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAgent {
    /// Array of `BrandVersion` objects identifying browsers/components. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<Vec<BrandVersion>>,
    /// `BrandVersion` object identifying platform/OS. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<BrandVersion>,
    /// 1 if agent prefers 'mobile', 0 if 'desktop'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<i64>,
    /// Device's major binary architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// Device's bitness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,
    /// Device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Source of data used to create this object. Refer to `AdCOM 1.0` List: User-Agent Source.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub source: Option<i64>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Identifies a device's browser/component or platform/OS using User-Agent Client Hints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrandVersion {
    /// Brand identifier.
    pub brand: String,
    /// Sequence of version components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Represents the impression multiplier for DOOH/CTV.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Qty {
    /// Quantity of billable events if purchased.
    pub multiplier: f64,
    /// Source type of quantity measurement. Refer to `AdCOM 1.0` List: Multiplier Measurement Source Types. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub sourcetype: Option<i64>,
    /// Top-level business domain of measurement vendor. Required if sourcetype=1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Details of the Digital Out of Home inventory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DOOH {
    /// Exchange provided id for placement/grouping. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of out-of-home venue. Default: `OpenOOH` Venue Taxonomy 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venuetype: Option<Vec<String>>,
    /// Venue taxonomy in use. Refer to `AdCOM 1.0` List: Venue Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub venuetypetax: Option<i64>,
    /// Details about the publisher of the placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,
    /// Domain of the inventory owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Comma separated list of keywords about the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// Details about the Content within the DOOH placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    /// Placeholder for exchange-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Details about ad slots being refreshed automatically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refresh {
    /// Array of `RefSettings` objects describing refresh mechanics. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refsettings: Option<Vec<RefSettings>>,
    /// Number of times this ad slot refreshed since last page load. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Information on how often and what triggers an ad slot refresh.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefSettings {
    /// Type of declared auto refresh. Refer to `AdCOM 1.0` List: Auto Refresh Triggers. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")] // Changed type, removed default
    pub reftype: Option<i64>,
    /// Minimum refresh interval in seconds. Recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minint: Option<i64>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Allows specifying price floors for video/audio creatives based on duration ranges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DurFloors {
    /// Low end of duration range (seconds). If missing, unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mindur: Option<i64>,
    /// High end of duration range (seconds). If missing, unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdur: Option<i64>,
    /// Minimum bid (CPM) for this duration range. Defaults to Imp.bidfloor if outside ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<f64>,
    /// Placeholder for vendor specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Top-level bid response object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidResponse {
    /// ID of the bid request to which this is a response.
    pub id: String,
    /// Array of `SeatBid` objects; 1+ required if bidding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<SeatBid>>,
    /// Bidder generated response ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidid: Option<String>,
    /// Bid currency using ISO-4217 alpha codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<String>,
    /// Optional feature to set data in exchange's cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,
    /// Reason for not bidding. Refer to `OpenRTB` 3.0 List: No-Bid Reason Codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbr: Option<i64>,
    /// Placeholder for bidder-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Collection of bids from a specific bidder seat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeatBid {
    /// Array of 1+ Bid objects.
    pub bid: Vec<Bid>,
    /// ID of the buyer seat on whose behalf this bid is made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,
    /// 0 = impressions can be won individually; 1 = must be won/lost as group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,
    /// Placeholder for bidder-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// An offer to buy a specific impression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bid {
    /// Bidder generated bid ID.
    pub id: String,
    /// ID of the Imp object in the related bid request.
    pub impid: String,
    /// Bid price expressed as CPM.
    pub price: f64,
    /// Win notice URL. Macros supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    /// Billing notice URL. Macros supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burl: Option<String>,
    /// Loss notice URL. Macros supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<String>,
    /// Optional ad markup. Supersedes win notice markup. Macros supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,
    /// ID of a preloaded ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adid: Option<String>,
    /// Advertiser domain for block list checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adomain: Option<Vec<String>>,
    /// Store ID of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,
    /// URL to image representative of campaign for ad quality/safety checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iurl: Option<String>,
    /// Campaign ID for ad quality checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    /// Creative ID for ad quality checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crid: Option<String>,
    /// Tactic ID for reporting. Meaning coordinated a priori.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tactic: Option<String>,
    /// Taxonomy in use for 'cat'. Refer to `AdCOM 1.0` List: Category Taxonomies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cattax: Option<i64>,
    /// IAB Tech Lab content categories of the creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    /// Set of attributes describing the creative. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<Vec<i64>>,
    /// List of supported APIs for the markup. Refer to `AdCOM 1.0` List: API Frameworks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<i64>>,
    /// NOTE: Deprecated in favor of apis.
    #[deprecated(note = "Deprecated in favor of apis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<i64>,
    /// Video response protocol. Refer to `AdCOM 1.0` List: Creative Subtypes - Audio/Video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<i64>,
    /// Creative media rating per IQG guidelines. Refer to `AdCOM 1.0` List: Media Ratings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<i64>,
    /// Language of creative using ISO-639-1-alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Language of creative using IETF BCP 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langb: Option<String>,
    /// Reference to deal.id if this bid pertains to a PMP deal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dealid: Option<String>,
    /// Width of the creative in DIPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
    /// Height of the creative in DIPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    /// Relative width of creative for ratio size (Flex Ads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i64>,
    /// Relative height of creative for ratio size (Flex Ads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i64>,
    /// Advisory seconds bidder willing to wait between auction and impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    /// Duration of video/audio creative in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<i64>,
    /// Type of creative markup: 1=Banner, 2=Video, 3=Audio, 4=Native.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtype: Option<i64>,
    /// Indicates bid eligibility for specific position within video/audio pod. Refer to `AdCOM 1.0` List: Slot Position in Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slotinpod: Option<i64>,
    /// Placeholder for bidder-specific extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
