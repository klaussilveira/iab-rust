#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Device Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DeviceType {
    MobileTablet,
    PersonalComputer,
    ConnectedTV,
    Phone,
    Tablet,
    ConnectedDevice,
    SetTopBox,
    OOHDevice,
    Unknown(i64),
}

impl From<i64> for DeviceType {
    fn from(value: i64) -> Self {
        match value {
            1 => DeviceType::MobileTablet,
            2 => DeviceType::PersonalComputer,
            3 => DeviceType::ConnectedTV,
            4 => DeviceType::Phone,
            5 => DeviceType::Tablet,
            6 => DeviceType::ConnectedDevice,
            7 => DeviceType::SetTopBox,
            8 => DeviceType::OOHDevice,
            _ => DeviceType::Unknown(value),
        }
    }
}

impl From<DeviceType> for i64 {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::MobileTablet => 1,
            DeviceType::PersonalComputer => 2,
            DeviceType::ConnectedTV => 3,
            DeviceType::Phone => 4,
            DeviceType::Tablet => 5,
            DeviceType::ConnectedDevice => 6,
            DeviceType::SetTopBox => 7,
            DeviceType::OOHDevice => 8,
            DeviceType::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DeviceType);
