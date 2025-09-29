use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{DoohVenueTaxonomy, DoohVenueType, DoohMultiplierMeasurementSourceType, CategoryTaxonomy};
use super::{publisher::Publisher, content::Content};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: DOOH
/// Derived from: DistributionChannel
/// This object is used to define an ad supported digital out-of-home (DOOH) experience
/// such as a digital billboard or digital signage. As a derived class, a DOOH object
/// inherits all DistributionChannel attributes and adds those defined below.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Dooh {
    /// Vendor-specific unique identifier of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Displayable name of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,

    /// Domain of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,

    /// Array of content categories describing the distribution channel using IDs from
    /// the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,

    /// Array of content categories describing the current section of the distribution
    /// channel using IDs from the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub sectcat: Option<Vec<String>>,

    /// Array of content categories describing the current page or view of the distribution
    /// channel using IDs from the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub pagecat: Option<Vec<String>>,

    /// The taxonomy in use for the cat, sectcat and pagecat attributes.
    /// Refer to List: Category Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CategoryTaxonomy>>"))]
    pub cattax: Option<CategoryTaxonomy>,

    /// Indicates if the distribution channel has a privacy policy, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub privpolicy: Option<i64>,

    /// The unique identifier of the out-of-home venue.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub venueid: Option<String>,

    /// Name of the out-of-home venue.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub venuename: Option<String>,

    /// The taxonomy in use for the venuetype attribute. Refer to List: DOOH Venue Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<DoohVenueTaxonomy>>"))]
    pub venuetax: Option<DoohVenueTaxonomy>,

    /// The type of out-of-home venue. The taxonomy is defined by the venuetax field.
    /// Refer to List: DOOH Venue Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<DoohVenueType>>"))]
    pub venuetype: Option<DoohVenueType>,

    /// The venue category using a taxonomy from the venuetax field.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub venuecat: Option<String>,

    /// Quality level indicator. This is vendor-specific.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub ql: Option<i64>,

    /// Multiplier which can be used to convert the "opportunity to see" estimated
    /// audience (i.e., passers-by) to the effective audience.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub mult: Option<f64>,

    /// The source of the multiplier value.
    /// Refer to List: DOOH Multiplier Measurement Source Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<DoohMultiplierMeasurementSourceType>>"))]
    pub multsrc: Option<DoohMultiplierMeasurementSourceType>,

    /// Comma separated list of keywords about the DOOH media. This field is deprecated,
    /// use 'kwarray' instead.
    #[deprecated(note="This field is deprecated, use 'kwarray' instead")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,

    /// Array of keywords about the DOOH media. Only one of 'keywords' or 'kwarray' may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,

    /// Details about the publisher of the DOOH placement. Refer to Object: Publisher.
    #[serde(rename="pub")]
    pub r#pub: Option<Publisher>,

    /// Details about the content being shown on the DOOH placement. Refer to Object: Content.
    pub content: Option<Content>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
