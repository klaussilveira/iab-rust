use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{EventType, EventTrackingMethod, ApiFramework};
use std::collections::HashMap;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Event
/// This object specifies a type of event that the advertiser or buying platform wants
/// to track along with the information required to do so.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Event {
    /// Type of event to track. Refer to List: Event Types.
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<EventType>"))]
    pub r#type: EventType,

    /// Method of tracking requested. Refer to List: Event Tracking Methods.
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<EventTrackingMethod>"))]
    pub method: EventTrackingMethod,

    /// The APIs being used by the tracker; only relevant when the tracking method
    /// is JavaScript. Refer to List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ApiFramework>>>"))]
    pub api: Option<Vec<ApiFramework>>,

    /// The URL of the tracking pixel or JavaScript tag, respectively.
    /// * Required for Image-Pixel or JavaScript methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub url: Option<String>,

    /// An array of key-value pairs to support vendor-specific data required for
    /// custom tracking. For example, the account number of a buyer with a tracking
    /// company might be represented as: {"acct": "123"}.
    #[cfg_attr(feature="coercion", serde_as(as="Option<HashMap<AsString, AsString>>"))]
    pub cdata: Option<HashMap<String, String>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
