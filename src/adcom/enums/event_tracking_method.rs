#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Event Tracking Methods
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum EventTrackingMethod {
    ImagePixel,
    JavaScript,
    Unknown(i64),
}

impl From<i64> for EventTrackingMethod {
    fn from(value: i64) -> Self {
        match value {
            1 => EventTrackingMethod::ImagePixel,
            2 => EventTrackingMethod::JavaScript,
            _ => EventTrackingMethod::Unknown(value),
        }
    }
}

impl From<EventTrackingMethod> for i64 {
    fn from(value: EventTrackingMethod) -> Self {
        match value {
            EventTrackingMethod::ImagePixel => 1,
            EventTrackingMethod::JavaScript => 2,
            EventTrackingMethod::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(EventTrackingMethod);
