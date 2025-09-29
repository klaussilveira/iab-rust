use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{Geo, Data, EID};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: User
/// This object contains information known or derived about the human user of the
/// device (i.e., the audience for advertising). The user `id` is an exchange
/// artifact and may be subject to rotation or other privacy policies. However,
/// when present, this user ID should be stable long enough to serve reasonably
/// as the basis for frequency capping and retargeting.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct User {
    /// Exchange-specific ID for the user. Unless prior arrangements have been
    /// made between the buyer and the seller directly, the value in this field
    /// is expected to be derived from an ID sync. (see Appendix: Cookie Based
    /// ID Syncing)
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,
    /// Buyer-specific ID for the user as mapped by the exchange for the buyer.
    /// Unless prior arrangements have been made between the buyer and the seller
    /// directly, the value in this field is expected to be derived from an ID
    /// sync. (see Appendix: Cookie Based ID Syncing)
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub buyeruid: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub yob: Option<i64>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub gender: Option<String>,
    /// Comma separated list of keywords, interests, or intent.
    /// Only one of `keywords` or `kwarray` may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,
    /// Array of keywords about the user.
    /// Only one of `keywords` or `kwarray` may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,
    /// Optional feature to pass bidder data that was set in the exchange's
    /// cookie. The string must be in base85 cookie safe characters and be
    /// in any format. Proper JSON encoding must be used to include "escaped"
    /// quotation marks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub customdata: Option<String>,
    /// Location of the user's home base defined by a `Geo` object
    /// (Section 3.2.19). This is not necessarily their current location.
    pub geo: Option<Geo>,
    /// Additional user data. Each `Data` object (Section 3.2.21) represents
    /// a different data source.
    pub data: Option<Vec<Data>>,
    /// When GDPR regulations are in effect this attribute contains the
    /// Transparency and Consent Framework's Consent String data structure.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub consent: Option<String>,
    /// Details for support of a standard protocol for multiple third party
    /// identity providers (Section 3.2.27).
    pub eids: Option<Vec<EID>>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
