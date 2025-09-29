#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Event Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum EventType {
    Impression,
    ViewableIMPMRC50,
    ViewableIMPMRC100,
    ViewableIMPVideoMRC50,
    Unknown(i64),
}

impl From<i64> for EventType {
    fn from(value: i64) -> Self {
        match value {
            1 => EventType::Impression,
            2 => EventType::ViewableIMPMRC50,
            3 => EventType::ViewableIMPMRC100,
            4 => EventType::ViewableIMPVideoMRC50,
            _ => EventType::Unknown(value),
        }
    }
}

impl From<EventType> for i64 {
    fn from(value: EventType) -> Self {
        match value {
            EventType::Impression => 1,
            EventType::ViewableIMPMRC50 => 2,
            EventType::ViewableIMPMRC100 => 3,
            EventType::ViewableIMPVideoMRC50 => 4,
            EventType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(EventType);
