use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{OperatingSystem, UserAgentSource};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: UserAgent
/// Structured user agent information if both the User-Agent header and the Sec-CH-UA-*
/// headers are available. If only the User-Agent header is available, this object should
/// be omitted and the device.ua field should be used.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct UserAgent {
    /// The user agent's execution platform or browser.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub browser: Option<String>,

    /// The user agent's execution platform or browser version.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub browserver: Option<String>,

    /// The user agent's operating system platform (e.g., "Windows", "Android", etc.).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub platform: Option<String>,

    /// The user agent's operating system. Refer to List: Operating Systems.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<OperatingSystem>>"))]
    pub os: Option<OperatingSystem>,

    /// The user agent's operating system version.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub osver: Option<String>,

    /// 1 if the user agent explicitly signals a preference for a "mobile" version
    /// of the content, if available; else 0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub mobile: Option<i64>,

    /// The user agent's underlying CPU architecture, such as "x86" or "arm".
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub arch: Option<String>,

    /// The user agent's device model. For example, "Pixel 2 XL".
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub model: Option<String>,

    /// The source of data used to create this object, as a UserAgentSource value.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<UserAgentSource>>"))]
    pub source: Option<UserAgentSource>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
