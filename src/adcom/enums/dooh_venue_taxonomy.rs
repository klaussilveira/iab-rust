#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: DOOH Venue Taxonomies
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum DoohVenueTaxonomy {
    AdcomDOOH1_0,
    OpenOOH1_0,
    DPAA,
    DMI,
    Unknown(i64),
}

impl From<i64> for DoohVenueTaxonomy {
    fn from(value: i64) -> Self {
        match value {
            0 => DoohVenueTaxonomy::AdcomDOOH1_0,
            1 => DoohVenueTaxonomy::OpenOOH1_0,
            2 => DoohVenueTaxonomy::DPAA,
            3 => DoohVenueTaxonomy::DMI,
            _ => DoohVenueTaxonomy::Unknown(value),
        }
    }
}

impl From<DoohVenueTaxonomy> for i64 {
    fn from(value: DoohVenueTaxonomy) -> Self {
        match value {
            DoohVenueTaxonomy::AdcomDOOH1_0 => 0,
            DoohVenueTaxonomy::OpenOOH1_0 => 1,
            DoohVenueTaxonomy::DPAA => 2,
            DoohVenueTaxonomy::DMI => 3,
            DoohVenueTaxonomy::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(DoohVenueTaxonomy);
