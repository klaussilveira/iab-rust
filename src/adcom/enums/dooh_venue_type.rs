#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: DOOH Venue Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DoohVenueType {
    Airborne,
    Airport,
    Cinema,
    ResidentialCommercial,
    Unknown(i64),
}

impl From<i64> for DoohVenueType {
    fn from(value: i64) -> Self {
        match value {
            1 => DoohVenueType::Airborne,
            2 => DoohVenueType::Airport,
            3 => DoohVenueType::Cinema,
            4 => DoohVenueType::ResidentialCommercial,
            _ => DoohVenueType::Unknown(value),
        }
    }
}

impl From<DoohVenueType> for i64 {
    fn from(value: DoohVenueType) -> Self {
        match value {
            DoohVenueType::Airborne => 1,
            DoohVenueType::Airport => 2,
            DoohVenueType::Cinema => 3,
            DoohVenueType::ResidentialCommercial => 4,
            DoohVenueType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DoohVenueType);
