use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{EventType, EventTrackingMethod, ApiFramework};
use crate::defaults::default_i64_one;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: EventSpec
/// This object specifies a type of ad tracking event and which methods of tracking are
/// available for it. This object may appear as an array for a given placement indicating
/// various types of available tracking events.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct EventSpec {
    /// Type of supported ad tracking event. Refer to List: Event Types.
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<EventType>"))]
    pub r#type: EventType,

    /// Array of supported event tracking methods for this event type.
    /// Refer to List: Event Tracking Methods.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<EventTrackingMethod>>>"))]
    pub method: Option<Vec<EventTrackingMethod>>,

    /// Event tracking APIs available for use; only relevant for JavaScript method trackers.
    /// Refer to List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ApiFramework>>>"))]
    pub api: Option<Vec<ApiFramework>>,

    /// Array of domains, top two levels only (e.g., "tracker.com"), that constitute
    /// a restriction list of JavaScript trackers. The sense of the restrictions is
    /// determined by wjs.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub jstrk: Option<Vec<String>>,

    /// Sense of the jstrk restriction list, where 0=block list, 1=allow list.
    #[serde(default="default_i64_one")]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub wjs: i64,

    /// Array of domains, top two levels only (e.g., "tracker.com"), that constitute
    /// a restriction list of pixel image trackers. The sense of the restrictions is
    /// determined by wpx.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub pxtrk: Option<Vec<String>>,

    /// Sense of the pxtrk restriction list, where 0=block list, 1=allow list.
    #[serde(default="default_i64_one")]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub wpx: i64,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
