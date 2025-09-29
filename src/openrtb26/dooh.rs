use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_venuetax_one;
use crate::adcom;
use super::{Publisher, Content};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Details of the Digital Out of Home inventory.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct DOOH {
    /// Exchange provided id for placement/grouping. Recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,
    /// Name of the DOOH placement.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub name: Option<String>,
    /// Type of out-of-home venue. Default: `OpenOOH` Venue Taxonomy 1.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub venuetype: Option<Vec<String>>,
    /// Venue taxonomy in use. Refer to `AdCOM 1.0` List: Venue Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::DoohVenueTaxonomy>>"))]
    #[serde(default="default_optional_venuetax_one")]
    pub venuetypetax: Option<adcom::enums::DoohVenueTaxonomy>,
    /// Details about the publisher of the placement.
    pub publisher: Option<Publisher>,
    /// Domain of the inventory owner.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub domain: Option<String>,
    /// Comma separated list of keywords about the DOOH placement.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,
    /// Details about the Content within the DOOH placement.
    pub content: Option<Content>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
