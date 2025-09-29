use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::CategoryTaxonomy;
use super::publisher::Publisher;
use super::content::Content;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: App
/// Derived from: DistributionChannel
/// This object is used to define an ad supported non-browser application, in contrast
/// to a typical website, for example. As a derived class, an App object inherits all
/// DistributionChannel attributes and adds those defined below.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct App {
    /// Vendor-specific unique identifier of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Displayable name of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,

    /// Domain of the app (e.g., "mygame.foo.com").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,

    /// Array of content categories describing the app using IDs from the taxonomy
    /// indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,

    /// Array of content categories describing the current section of the app using
    /// IDs from the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub sectcat: Option<Vec<String>>,

    /// Array of content categories describing the current page or view of the app
    /// using IDs from the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub pagecat: Option<Vec<String>>,

    /// The taxonomy in use for the cat, sectcat and pagecat attributes.
    /// Refer to List: Category Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CategoryTaxonomy>>"))]
    pub cattax: Option<CategoryTaxonomy>,

    /// Indicates if the app has a privacy policy, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub privpolicy: Option<i64>,

    /// Comma separated list of keywords about the app. This field is deprecated,
    /// use 'kwarray' instead.
    #[deprecated(note="This field is deprecated, use 'kwarray' instead")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,

    /// Array of keywords about the app. Only one of 'keywords' or 'kwarray' may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,

    /// Bundle or package name of the app (e.g., "com.foo.mygame") and should NOT be
    /// app store IDs (e.g., not iTunes store IDs).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub bundle: Option<String>,

    /// The ID of the app in an app store (e.g., Apple iTunes, Google Play).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub storeid: Option<String>,

    /// App store URL for an installed app; for IQG 2.1 compliance.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub storeurl: Option<String>,

    /// Application version.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ver: Option<String>,

    /// Indicator of whether or not this is a paid app, where 0=free, 1=paid.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub paid: i64,

    /// Details about the publisher of the distribution channel. Refer to Object: Publisher.
    #[serde(rename="pub")]
    pub r#pub: Option<Publisher>,

    /// Details about the content within the distribution channel. Refer to Object: Content.
    pub content: Option<Content>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
