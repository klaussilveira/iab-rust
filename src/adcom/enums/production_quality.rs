#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Production Quality
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum ProductionQuality {
    Unknown(i64),
    Professional,
    Prosumer,
    UserGenerated,
}

impl From<i64> for ProductionQuality {
    fn from(value: i64) -> Self {
        match value {
            0 => ProductionQuality::Unknown(0),
            1 => ProductionQuality::Professional,
            2 => ProductionQuality::Prosumer,
            3 => ProductionQuality::UserGenerated,
            _ => ProductionQuality::Unknown(value),
        }
    }
}

impl From<ProductionQuality> for i64 {
    fn from(value: ProductionQuality) -> Self {
        match value {
            ProductionQuality::Professional => 1,
            ProductionQuality::Prosumer => 2,
            ProductionQuality::UserGenerated => 3,
            ProductionQuality::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(ProductionQuality);
