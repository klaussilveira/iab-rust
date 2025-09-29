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

/// Object: Site
/// Derived from: DistributionChannel
/// This object is used to define an ad supported website, in contrast to a non-browser
/// application, for example. As a derived class, a Site object inherits all
/// DistributionChannel attributes and adds those defined below.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Site {
    /// Vendor-specific unique identifier of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Displayable name of the distribution channel.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,

    /// Domain of the site (e.g., "mysite.foo.com").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,

    /// Array of content categories describing the site using IDs from the taxonomy
    /// indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,

    /// Array of content categories describing the current section of the site using
    /// IDs from the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub sectcat: Option<Vec<String>>,

    /// Array of content categories describing the current page or view of the site
    /// using IDs from the taxonomy indicated in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub pagecat: Option<Vec<String>>,

    /// The taxonomy in use for the cat, sectcat and pagecat attributes.
    /// Refer to List: Category Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CategoryTaxonomy>>"))]
    pub cattax: Option<CategoryTaxonomy>,

    /// Indicates if the site has a privacy policy, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub privpolicy: Option<i64>,

    /// Comma separated list of keywords about the site. This field is deprecated,
    /// use 'kwarray' instead.
    #[deprecated(note="This field is deprecated, use 'kwarray' instead")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,

    /// Array of keywords about the site. Only one of 'keywords' or 'kwarray' may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,

    /// URL of the page within the site.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub page: Option<String>,

    /// Referrer URL that caused navigation to the current page.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub r#ref: Option<String>,

    /// Search string that caused navigation to the current page.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub search: Option<String>,

    /// Indicates if the site has been programmed to optimize layout when viewed on
    /// mobile devices, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub mobile: Option<i64>,

    /// Indicates if the page is built with AMP HTML, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub amp: Option<i64>,

    /// Details about the publisher of the distribution channel. Refer to Object: Publisher.
    #[serde(rename="pub")]
    pub r#pub: Option<Publisher>,

    /// Details about the content within the distribution channel. Refer to Object: Content.
    pub content: Option<Content>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
