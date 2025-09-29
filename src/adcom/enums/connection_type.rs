#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Connection Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum ConnectionType {
    Unknown(i64),
    Ethernet,
    Wifi,
    CellularUnknown,
    Cellular2G,
    Cellular3G,
    Cellular4G,
    Cellular5G,
}

impl From<i64> for ConnectionType {
    fn from(value: i64) -> Self {
        match value {
            0 => ConnectionType::Unknown(0),
            1 => ConnectionType::Ethernet,
            2 => ConnectionType::Wifi,
            3 => ConnectionType::CellularUnknown,
            4 => ConnectionType::Cellular2G,
            5 => ConnectionType::Cellular3G,
            6 => ConnectionType::Cellular4G,
            7 => ConnectionType::Cellular5G,
            _ => ConnectionType::Unknown(value),
        }
    }
}

impl From<ConnectionType> for i64 {
    fn from(value: ConnectionType) -> Self {
        match value {
            ConnectionType::Ethernet => 1,
            ConnectionType::Wifi => 2,
            ConnectionType::CellularUnknown => 3,
            ConnectionType::Cellular2G => 4,
            ConnectionType::Cellular3G => 5,
            ConnectionType::Cellular4G => 6,
            ConnectionType::Cellular5G => 7,
            ConnectionType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(ConnectionType);
