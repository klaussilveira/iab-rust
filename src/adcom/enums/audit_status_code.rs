#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Audit Status Codes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum AuditStatusCode {
    PendingAudit,
    PreApproved,
    Approved,
    Denied,
    ChangedResubmit,
    Unknown(i64),
}

impl From<i64> for AuditStatusCode {
    fn from(value: i64) -> Self {
        match value {
            1 => AuditStatusCode::PendingAudit,
            2 => AuditStatusCode::PreApproved,
            3 => AuditStatusCode::Approved,
            4 => AuditStatusCode::Denied,
            5 => AuditStatusCode::ChangedResubmit,
            _ => AuditStatusCode::Unknown(value),
        }
    }
}

impl From<AuditStatusCode> for i64 {
    fn from(value: AuditStatusCode) -> Self {
        match value {
            AuditStatusCode::PendingAudit => 1,
            AuditStatusCode::PreApproved => 2,
            AuditStatusCode::Approved => 3,
            AuditStatusCode::Denied => 4,
            AuditStatusCode::ChangedResubmit => 5,
            AuditStatusCode::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(AuditStatusCode);
