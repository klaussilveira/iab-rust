#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Delivery Methods
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DeliveryMethod {
    Streaming,
    Progressive,
    Download,
    Unknown(i64),
}

impl From<i64> for DeliveryMethod {
    fn from(value: i64) -> Self {
        match value {
            1 => DeliveryMethod::Streaming,
            2 => DeliveryMethod::Progressive,
            3 => DeliveryMethod::Download,
            _ => DeliveryMethod::Unknown(value),
        }
    }
}

impl From<DeliveryMethod> for i64 {
    fn from(value: DeliveryMethod) -> Self {
        match value {
            DeliveryMethod::Streaming => 1,
            DeliveryMethod::Progressive => 2,
            DeliveryMethod::Download => 3,
            DeliveryMethod::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DeliveryMethod);
