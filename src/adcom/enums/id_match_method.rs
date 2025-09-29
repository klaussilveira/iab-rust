#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: ID Match Methods
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum IdMatchMethod {
    Deterministic,
    Probabilistic,
    MixedDeterministicProbabilistic,
    Unknown(i64),
}

impl From<i64> for IdMatchMethod {
    fn from(value: i64) -> Self {
        match value {
            1 => IdMatchMethod::Deterministic,
            2 => IdMatchMethod::Probabilistic,
            3 => IdMatchMethod::MixedDeterministicProbabilistic,
            _ => IdMatchMethod::Unknown(value),
        }
    }
}

impl From<IdMatchMethod> for i64 {
    fn from(value: IdMatchMethod) -> Self {
        match value {
            IdMatchMethod::Deterministic => 1,
            IdMatchMethod::Probabilistic => 2,
            IdMatchMethod::MixedDeterministicProbabilistic => 3,
            IdMatchMethod::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(IdMatchMethod);
