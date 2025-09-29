use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{display::Display, video::Video, audio::Audio, audit::Audit, enums::{CategoryTaxonomy, MediaRating, CreativeAttribute}};
use crate::defaults::default_cattax_two;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Ad
/// This object is the root of a structure that defines an instance of advertising media.
/// It includes metadata about the ad overall and sub-objects that provide additional
/// detail specific to the type of media comprising the creative.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Ad {
    /// ID of the creative; unique at least throughout the scope of a vendor
    /// (e.g., an exchange or buying platform). Note that multiple instances
    /// of the same ad when used in transactions must have the same ID.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,

    /// Advertiser domain; top two levels only (e.g., "ford.com").
    /// This can be an array for the case of rotating creatives.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub adomain: Option<Vec<String>>,

    /// When the product of the ad is an app, the unique ID of that app as a
    /// bundle or package name (e.g., "com.foo.mygame"). This should NOT be
    /// an app store ID (e.g., no iTunes store IDs). This can be an array
    /// for the case of rotating creatives.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub bundle: Option<Vec<String>>,

    /// URL without cache-busting to an image that is representative of the
    /// ad content for cursory level ad quality checking.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub iurl: Option<String>,

    /// Array of content categories describing the ad using IDs from the
    /// taxonomy indicated in cattax. Implementer should ensure compliance
    /// with regional legislation around data usage and sharing.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,

    /// The taxonomy in use for the cat attribute. Refer to List: Category Taxonomies.
    #[serde(default="default_cattax_two")]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<CategoryTaxonomy>"))]
    pub cattax: CategoryTaxonomy,

    /// Language of the creative using ISO-639-1-alpha-2. In practice, vendors
    /// using this object may elect an alternate standard (e.g., BCP-47) in which
    /// case this must be communicated beforehand. The non-standard code "xx" may
    /// also be used if the creative has no linguistic content (e.g., a banner
    /// with just a company logo).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub lang: Option<String>,

    /// Set of attributes describing the creative. Refer to List: Creative Attributes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<CreativeAttribute>>>"))]
    pub attr: Option<Vec<CreativeAttribute>>,

    /// Flag to indicate if the creative is secure (i.e., uses HTTPS for all assets
    /// and markup), where 0=no, 1=yes. There is no default and thus if omitted,
    /// the secure state is unknown. However, as a practical matter, the safe
    /// assumption is to treat unknown as non-secure.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub secure: Option<i64>,

    /// Media rating per IQG guidelines. Refer to List: Media Ratings.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<MediaRating>>"))]
    pub mrating: Option<MediaRating>,

    /// Timestamp of the original instantiation of this ad (i.e., this object
    /// or any of its children) in Unix format (i.e., milliseconds since the epoch).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub init: Option<i64>,

    /// Timestamp of most recent modification to this ad (i.e., this object or
    /// any of its children other than the Audit object) in Unix format
    /// (i.e., milliseconds since the epoch).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub lastmod: Option<i64>,

    /// Media Subtype Object that indicates this is a display ad and provides
    /// additional detail as such. Refer to Object: Display.
    /// * Required if no other media subtype object is specified.
    pub display: Option<Display>,

    /// Media Subtype Object that indicates this is a video ad and provides
    /// additional detail as such. Refer to Object: Video.
    /// * Required if no other media subtype object is specified.
    pub video: Option<Video>,

    /// Media Subtype Object that indicates this is an audio ad and provides
    /// additional detail as such. Refer to Object: Audio.
    /// * Required if no other media subtype object is specified.
    pub audio: Option<Audio>,

    /// An object depicting the audit status of the ad; typically part of a
    /// quality/safety review process. Refer to Object: Audit.
    pub audit: Option<Audit>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
