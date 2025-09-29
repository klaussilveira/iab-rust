#[cfg(feature="utoipa")]
use utoipa::ToSchema;

#[allow(non_camel_case_types)]
/// List: Operating Systems
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum OperatingSystem {
    OtherNotListed,
    Windows3_1_x,
    Windows95,
    Windows98,
    WindowsNT,
    Windows2000,
    WindowsXP,
    WindowsVista,
    Windows7,
    Windows8,
    Windows8_1,
    Windows10,
    Windows11,
    MacOS,
    Linux,
    Ubuntu,
    iOS,
    AndroidOS,
    WindowsPhone7,
    WindowsPhone8,
    WindowsPhone8_1,
    WindowsMobile,
    ChromeOS,
    Unknown(i64),
}

impl From<i64> for OperatingSystem {
    fn from(value: i64) -> Self {
        match value {
            0 => OperatingSystem::OtherNotListed,
            1 => OperatingSystem::Windows3_1_x,
            2 => OperatingSystem::Windows95,
            3 => OperatingSystem::Windows98,
            4 => OperatingSystem::WindowsNT,
            5 => OperatingSystem::Windows2000,
            6 => OperatingSystem::WindowsXP,
            7 => OperatingSystem::WindowsVista,
            8 => OperatingSystem::Windows7,
            9 => OperatingSystem::Windows8,
            10 => OperatingSystem::Windows8_1,
            11 => OperatingSystem::Windows10,
            30 => OperatingSystem::Windows11,
            12 => OperatingSystem::MacOS,
            13 => OperatingSystem::Linux,
            14 => OperatingSystem::Ubuntu,
            15 => OperatingSystem::iOS,
            16 => OperatingSystem::AndroidOS,
            17 => OperatingSystem::WindowsPhone7,
            18 => OperatingSystem::WindowsPhone8,
            19 => OperatingSystem::WindowsPhone8_1,
            20 => OperatingSystem::WindowsMobile,
            21 => OperatingSystem::ChromeOS,
            _ => OperatingSystem::Unknown(value),
        }
    }
}

impl From<OperatingSystem> for i64 {
    fn from(value: OperatingSystem) -> Self {
        match value {
            OperatingSystem::OtherNotListed => 0,
            OperatingSystem::Windows3_1_x => 1,
            OperatingSystem::Windows95 => 2,
            OperatingSystem::Windows98 => 3,
            OperatingSystem::WindowsNT => 4,
            OperatingSystem::Windows2000 => 5,
            OperatingSystem::WindowsXP => 6,
            OperatingSystem::WindowsVista => 7,
            OperatingSystem::Windows7 => 8,
            OperatingSystem::Windows8 => 9,
            OperatingSystem::Windows8_1 => 10,
            OperatingSystem::Windows10 => 11,
            OperatingSystem::Windows11 => 30,
            OperatingSystem::MacOS => 12,
            OperatingSystem::Linux => 13,
            OperatingSystem::Ubuntu => 14,
            OperatingSystem::iOS => 15,
            OperatingSystem::AndroidOS => 16,
            OperatingSystem::WindowsPhone7 => 17,
            OperatingSystem::WindowsPhone8 => 18,
            OperatingSystem::WindowsPhone8_1 => 19,
            OperatingSystem::WindowsMobile => 20,
            OperatingSystem::ChromeOS => 21,
            OperatingSystem::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(OperatingSystem);
