#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Location Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum LocationType {
    GPS,
    IPAddress,
    UserProvided,
    Unknown(i64),
}

impl From<i64> for LocationType {
    fn from(value: i64) -> Self {
        match value {
            1 => LocationType::GPS,
            2 => LocationType::IPAddress,
            3 => LocationType::UserProvided,
            _ => LocationType::Unknown(value),
        }
    }
}

impl From<LocationType> for i64 {
    fn from(value: LocationType) -> Self {
        match value {
            LocationType::GPS => 1,
            LocationType::IPAddress => 2,
            LocationType::UserProvided => 3,
            LocationType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(LocationType);
