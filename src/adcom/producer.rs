use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::CategoryTaxonomy;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Producer
/// This object defines the producer of the content in which the ad will be shown. This is
/// particularly useful when the content is syndicated and may be distributed through
/// different channels and thus when the producer and publisher are not necessarily the
/// same entity.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Producer {
    /// Vendor-specific unique producer identifier. Useful if content is syndicated
    /// and may be posted on a site using multiple sellers. Can be used to look up seller.json.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Displayable name of the producer.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,

    /// The taxonomy in use. Refer to List: Category Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CategoryTaxonomy>>"))]
    pub cattax: Option<CategoryTaxonomy>,

    /// Array of content categories that describe the producer using the taxonomy
    /// identified in cattax.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the producer (e.g., "producer.com").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
