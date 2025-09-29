// AdCOM 1.0 Enumerations
#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// 5.2 API Frameworks
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum ApiFramework {
    VPAID1_0,
    VPAID2_0,
    MRAID1_0,
    ORMMA,
    MRAID2_0,
    MRAID3_0,
    OMID1_0,
    SIMID1_0,
    SIMID1_1,
    Unknown(i64),
}

impl From<i64> for ApiFramework {
    fn from(value: i64) -> Self {
        match value {
            1 => ApiFramework::VPAID1_0,
            2 => ApiFramework::VPAID2_0,
            3 => ApiFramework::MRAID1_0,
            4 => ApiFramework::ORMMA,
            5 => ApiFramework::MRAID2_0,
            6 => ApiFramework::MRAID3_0,
            7 => ApiFramework::OMID1_0,
            8 => ApiFramework::SIMID1_0,
            9 => ApiFramework::SIMID1_1,
            _ => ApiFramework::Unknown(value),
        }
    }
}

impl From<ApiFramework> for i64 {
    fn from(value: ApiFramework) -> Self {
        match value {
            ApiFramework::VPAID1_0 => 1,
            ApiFramework::VPAID2_0 => 2,
            ApiFramework::MRAID1_0 => 3,
            ApiFramework::ORMMA => 4,
            ApiFramework::MRAID2_0 => 5,
            ApiFramework::MRAID3_0 => 6,
            ApiFramework::OMID1_0 => 7,
            ApiFramework::SIMID1_0 => 8,
            ApiFramework::SIMID1_1 => 9,
            ApiFramework::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(ApiFramework);
