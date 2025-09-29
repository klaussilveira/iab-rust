use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::{default_optional_i64_zero, default_optional_i64_two, default_optional_cattax_one};
use crate::adcom;
use super::{Imp, Site, App, DOOH, Device, User, Source, Regs};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: BidRequest
/// The top-level bid request object contains an exchange unique bid request or
/// auction ID. This `id` attribute is required as is at least one impression object
/// (Section 3.2.4). Other attributes in this top-level object establish rules and
/// restrictions that apply to all impressions being offered.
#[skip_serializing_none]
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
#[cfg_attr(feature="utoipa", schema(
    description="OpenRTB Bid Request - Complete specification for requesting bids on ad inventory",
    example=json!({
        "id": "test-request-123",
        "imp": [{
            "id": "imp-1",
            "banner": {
                "w": 320,
                "h": 50
            },
            "bidfloor": 0.25
        }]
    })
))]
pub struct BidRequest {
    /// ID of the bid request, assigned by the exchange, and unique for the
    /// exchange's subsequent tracking of the responses. The exchange may use
    /// different values for different recipients.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,
    /// Array of `Imp` objects (Section 3.2.4) representing the impressions
    /// offered. At least 1 `Imp` object is required.
    pub imp: Vec<Imp>,
    /// Details via a `Site` object (Section 3.2.13) about the publisher's
    /// website. Only applicable and recommended for websites.
    pub site: Option<Site>,
    /// Details via an `App` object (Section 3.2.14) about the publisher's
    /// app (i.e., non-browser applications). Only applicable and recommended
    /// for apps.
    pub app: Option<App>,
    /// This object should be included if the ad supported content is a Digital
    /// Out-Of-Home screen. A bid request with a DOOH object must not contain
    /// a site or app object.
    pub dooh: Option<DOOH>,
    /// Details via a `Device` object (Section 3.2.18) about the user's device
    /// to which the impression will be delivered.
    pub device: Option<Device>,
    /// Details via a `User` object (Section 3.2.20) about the human user of
    /// the device; the advertising audience.
    pub user: Option<User>,
    /// Indicator of test mode in which auctions are not billable,
    /// where 0=live mode, 1=test mode.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub test: Option<i64>,
    /// Auction type, where 1=First Price, 2=Second Price Plus.
    /// Exchange-specific auction types can be defined using values 500
    /// and greater.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_two")]
    pub at: Option<i64>,
    /// Maximum time in milliseconds the exchange allows for bids to be received
    /// including Internet latency to avoid timeout. This value supersedes any
    /// *a priori* guidance from the exchange.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub tmax: Option<i64>,
    /// Allowed list of buyer seats (e.g., advertisers, agencies) allowed to bid
    /// on this impression. IDs of seats and knowledge of the buyer's customers
    /// to which they refer must be coordinated between bidders and the exchange
    /// *a priori*. At most, only one of `wseat` and `bseat` should be used in
    /// the same request. Omission of both implies no seat restrictions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub wseat: Option<Vec<String>>,
    /// Block list of buyer seats (e.g., advertisers, agencies) restricted from
    /// bidding on this impression. IDs of seats and knowledge of the buyer's
    /// customers to which they refer must be coordinated between bidders and
    /// the exchange *a priori*. At most, only one of `wseat` and `bseat` should
    /// be used in the same request. Omission of both implies no seat restrictions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub bseat: Option<Vec<String>>,
    /// Flag to indicate if Exchange can verify that the impressions offered
    /// represent all of the impressions available in context (e.g., all on the
    /// web page, all video spots such as pre/mid/post roll) to support
    /// road-blocking. 0=no or unknown, 1=yes, the impressions offered
    /// represent all that are available.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    #[serde(default="default_optional_i64_zero")]
    pub allimps: Option<i64>,
    /// Array of allowed currencies for bids on this bid request using ISO-4217
    /// alpha codes. Recommended only if the exchange accepts multiple currencies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cur: Option<Vec<String>>,
    /// Allowed list of languages for creatives using ISO-639-1-alpha-2.
    /// Omission implies no specific restrictions, but buyers would be advised
    /// to consider `language` attribute in the `Device` and/or `Content` objects
    /// if available. Only one of `wlang` or `wlangb` should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub wlang: Option<Vec<String>>,
    /// Allowed list of languages for creatives using IETF BCP 47I. Omission
    /// implies no specific restrictions, but buyers would be advised to consider
    /// language attribute in the `Device` and/or `Content` objects if available.
    /// Only one of `wlang` or `wlangb` should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub wlangb: Option<Vec<String>>,
    /// Allowed advertiser categories using the specified category taxonomy.
    /// The taxonomy to be used is defined by the `cattax` field. If no `cattax`
    /// field is supplied IAB Content Taxonomy 1.0 is assumed. Only one of `acat`
    /// or `bcat` should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub acat: Option<Vec<String>>,
    /// Blocked advertiser categories using the specified category taxonomy.
    /// The taxonomy to be used is defined by the `cattax` field. If no `cattax`
    /// field is supplied IAB Content Taxonomy 1.0 is assumed. Only one of `acat`
    /// or `bcat` should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub bcat: Option<Vec<String>>,
    /// The taxonomy in use for bcat. Refer to the AdCOM 1.0 list
    /// List: Category Taxonomies for values.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::CategoryTaxonomy>>"))]
    #[serde(default="default_optional_cattax_one")]
    pub cattax: Option<adcom::enums::CategoryTaxonomy>,
    /// Block list of advertisers by their domains (e.g., "ford.com").
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub badv: Option<Vec<String>>,
    /// Block list of applications by their app store IDs. See OTT/CTV Store
    /// Assigned App Identification Guidelines for more details about expected
    /// strings for CTV app stores. For mobile apps in Google Play Store, these
    /// should be bundle or package names (e.g. com.foo.mygame). For apps in
    /// Apple App Store, these should be a numeric ID.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub bapp: Option<Vec<String>>,
    /// A Source object (Section 3.2.2) that provides data about the inventory
    /// source and which entity makes the final decision.
    pub source: Option<Source>,
    /// A Regs object (Section 3.2.3) that specifies any industry, legal, or
    /// governmental regulations in force for this request.
    pub regs: Option<Regs>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
