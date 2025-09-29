use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::geo::Geo;
use super::data::Data;
use super::extended_identifiers::ExtendedIdentifiers;

#[cfg(feature="coercion")]
use crate::json_coercion::AsString;
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: User
/// This object contains information known or derived about the human user of the device
/// (i.e., the audience for advertising). The user ID is a vendor-specific artifact and
/// may be subject to rotation or other privacy policies. However, this user ID must be
/// stable long enough to serve reasonably as the basis for frequency capping and retargeting.
///
/// Implementer should ensure compliance with regional legislation around data usage and sharing.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct User {
    /// Vendor-specific ID for the user. At least one of id or buyeruid is strongly recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Buyer-specific ID for the user as mapped by an exchange for the buyer.
    /// At least one of id or buyeruid is strongly recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub buyeruid: Option<String>,

    /// Year of birth as a 4-digit integer. DEPRECATED as of OpenRTB 2.6
    #[deprecated(note="Deprecated as of OpenRTB 2.6")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub yob: Option<String>,

    /// Gender, where "M"=male, "F"=female, "O"=known to be other (i.e., omitted is unknown).
    /// DEPRECATED as of OpenRTB 2.6
    #[deprecated(note="Deprecated as of OpenRTB 2.6")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub gender: Option<String>,

    /// Comma separated list of keywords, interests, or intent. This field is deprecated, use 'kwarray' instead.
    #[deprecated(note="This field is deprecated, use 'kwarray' instead")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,

    /// Array of keywords describing the content. Only one of 'keywords' or 'kwarray' may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,

    /// GDPR consent string if applicable, complying with the IAB standard Consent String Format
    /// in the Transparency and Consent Framework technical specifications.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub consent: Option<String>,

    /// Location of the user's home base (i.e., not necessarily their current location).
    /// Refer to Object: Geo.
    pub geo: Option<Geo>,

    /// Additional user data. Each Data object represents a different data source.
    /// Refer to Object: Data.
    pub data: Option<Vec<Data>>,

    /// Extended (third-party) identifiers for this user. Refer to Object: Extended Identifiers.
    pub eids: Option<Vec<ExtendedIdentifiers>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
