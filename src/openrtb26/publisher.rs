use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_cattax_one;
use crate::adcom;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Entity who directly supplies inventory to and is paid by the exchange.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Publisher {
    /// Exchange-specific seller ID (corresponds to `seller_id` in sellers.json).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,
    /// Seller name.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,
    /// Taxonomy in use for categories. Refer to `AdCOM` List: Category Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::CategoryTaxonomy>>"))]
    #[serde(default="default_optional_cattax_one")]
    pub cattax: Option<adcom::enums::CategoryTaxonomy>,
    /// Array of IAB Tech Lab content categories of the publisher.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,
    /// Highest level domain of the seller.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
