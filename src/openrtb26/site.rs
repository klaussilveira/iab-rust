use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_cattax_one;
use crate::adcom;
use super::{Publisher, Content};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Site
/// This object should be included if the ad supported content is a website as
/// opposed to a non-browser application or Digital Out of Home (DOOH) inventory.
/// A bid request must not contain more than one of a `Site`, `App` or `DOOH` object.
/// At a minimum, it is useful to provide a site ID or page URL, but this is not
/// strictly required.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Site {
    /// Exchange-specific site ID.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,
    /// Site name (may be aliased at the publisher's request).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,
    /// Domain of the site (e.g., "mysite.foo.com").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,
    /// The taxonomy in use. Refer to the AdCOM List: Category Taxonomies
    /// for values. If no cattax field is supplied IAB Content Category
    /// Taxonomy 1.0 is assumed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::CategoryTaxonomy>>"))]
    #[serde(default="default_optional_cattax_one")]
    pub cattax: Option<adcom::enums::CategoryTaxonomy>,
    /// Array of IAB Tech Lab content categories of the site.
    /// The taxonomy to be used is defined by the cattax field.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,
    /// Array of IAB Tech Lab content categories that describe the current
    /// section of the site. The taxonomy to be used is defined by the
    /// cattax field.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub sectioncat: Option<Vec<String>>,
    /// Array of IAB Tech Lab content categories that describe the current
    /// page or view of the site. The taxonomy to be used is defined by
    /// the cattax field.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub pagecat: Option<Vec<String>>,
    /// URL of the page where the impression will be shown.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub page: Option<String>,
    /// Referrer URL that caused navigation to the current page.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    #[serde(rename="ref")]
    pub refer: Option<String>,
    /// Search string that caused navigation to the current page.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub search: Option<String>,
    /// Indicates if the site has been programmed to optimize layout when
    /// viewed on mobile devices, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub mobile: Option<i64>,
    /// Indicates if the site has a privacy policy, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub privacypolicy: Option<i64>,
    /// Details about the Publisher (Section 3.2.15) of the site.
    pub publisher: Option<Publisher>,
    /// Details about the Content (Section 3.2.16) within the site.
    pub content: Option<Content>,
    /// Comma separated list of keywords about the site.
    /// Only one of `keywords` or `kwarray` may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,
    /// Array of keywords about the site.
    /// Only one of `keywords` or `kwarray` may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,
    /// A domain to be used for inventory authorization in the case of
    /// inventory sharing arrangements between a site owner and content owner.
    /// This field is typically used by authorization crawlers to establish
    /// the domain of the content owner, who has the right to monetize some
    /// portion of ad inventory within the site. The content owner's domain
    /// should be listed in the site owner's ads.txt file as an
    /// `inventorypartnerdomain`. Authorization for supply from the
    /// `inventorypartnerdomain` will be published in the ads.txt file on
    /// the root of that domain. Refer to the ads.txt 1.1 spec for more details.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub inventorypartnerdomain: Option<String>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
