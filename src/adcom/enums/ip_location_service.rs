#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: IP Location Services
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum IpLocationService {
    IP2Location,
    Neustar,
    MaxMind,
    NetAcuity,
    Unknown(i64),
}

impl From<i64> for IpLocationService {
    fn from(value: i64) -> Self {
        match value {
            1 => IpLocationService::IP2Location,
            2 => IpLocationService::Neustar,
            3 => IpLocationService::MaxMind,
            4 => IpLocationService::NetAcuity,
            _ => IpLocationService::Unknown(value),
        }
    }
}

impl From<IpLocationService> for i64 {
    fn from(value: IpLocationService) -> Self {
        match value {
            IpLocationService::IP2Location => 1,
            IpLocationService::Neustar => 2,
            IpLocationService::MaxMind => 3,
            IpLocationService::NetAcuity => 4,
            IpLocationService::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(IpLocationService);
