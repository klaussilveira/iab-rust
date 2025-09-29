use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::adcom;
use super::{Geo, UserAgent};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Device
/// This object provides information pertaining to the device through which the user
/// is interacting. Device information includes its hardware, platform, location,
/// and carrier data. The device can refer to a mobile handset, a desktop computer,
/// set top box, or other digital device.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Device {
    /// Location of the device assumed to be the user's current location
    /// defined by a `Geo` object (Section 3.2.19).
    pub geo: Option<Geo>,
    /// Standard "Do Not Track" flag as set in the header by the browser,
    /// where 0=tracking is unrestricted, 1=do not track.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub dnt: Option<i64>,
    /// "Limit Ad Tracking" signal commercially endorsed (e.g., iOS, Android),
    /// where 0=tracking is unrestricted, 1=tracking must be limited per
    /// commercial guidelines.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub lmt: Option<i64>,
    /// Browser user agent string. This field represents a raw user agent
    /// string from the browser. For backwards compatibility, exchanges are
    /// recommended to always populate `ua` with the User-Agent string, when
    /// available from the end user's device, even if an alternative
    /// representation, such as the User-Agent Client-Hints, is available
    /// and is used to populate `sua`. No inferred or approximated user agents
    /// are expected in this field. If a client supports User-Agent Client Hints,
    /// and `sua` field is present, bidders are recommended to rely on `sua` for
    /// detecting device type, browser type and version and other purposes that
    /// rely on the user agent information, and ignore `ua` field. This is because
    /// the `ua` may contain a frozen or reduced user agent string.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ua: Option<String>,
    /// Structured user agent information defined by a `UserAgent` object
    /// (see Section 3.2.29). If both `ua` and `sua` are present in the bid
    /// request, `sua` should be considered the more accurate representation
    /// of the device attributes. This is because the `ua` may contain a
    /// frozen or reduced user agent string.
    pub sua: Option<UserAgent>,
    /// IPv4 address closest to device.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ip: Option<String>,
    /// IP address closest to device as IPv6.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ipv6: Option<String>,
    /// The general type of device. Refer to List: Device Types in AdCOM 1.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::DeviceType>>"))]
    pub devicetype: Option<adcom::enums::DeviceType>,
    /// Device make (e.g., "Apple").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub make: Option<String>,
    /// Device model (e.g., "iPhone").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub model: Option<String>,
    /// Device operating system (e.g., "iOS").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub os: Option<String>,
    /// Device operating system version (e.g., "3.1.2").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub osv: Option<String>,
    /// Hardware version of the device (e.g., "5S" for iPhone 5S).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub hwv: Option<String>,
    /// Physical height of the screen in pixels.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,
    /// Physical width of the screen in pixels.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,
    /// Screen size as pixels per linear inch.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub ppi: Option<i64>,
    /// The ratio of physical pixels to device-independent pixels (DIPS).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub pxratio: Option<f64>,
    /// Support for JavaScript, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub js: Option<i64>,
    /// Indicates if the geolocation API will be available to JavaScript
    /// code running in the banner, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub geofetch: Option<i64>,
    /// Version of Flash supported by the browser.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub flashver: Option<String>,
    /// Browser language using ISO-639-1-alpha-2.
    /// Only one of `language` or `langb` should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub language: Option<String>,
    /// Browser language using IETF BCP 47.
    /// Only one of `language` or `langb` should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub langb: Option<String>,
    /// Carrier or ISP (e.g., "VERIZON") using exchange curated string names
    /// which should be published to bidders *a priori*.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub carrier: Option<String>,
    /// Mobile carrier as the concatenated MCC-MNC code (e.g., "310-005"
    /// identifies Verizon Wireless CDMA in the USA). Refer to
    /// https://en.wikipedia.org/wiki/Mobile_country_code for further examples.
    /// Note that the dash between the MCC and MNC parts is required to remove
    /// parsing ambiguity. The MCC-MNC values represent the SIM installed on
    /// the device and do not change when a device is roaming. Roaming may be
    /// inferred by a combination of the MCC-MNC, geo, IP and other data signals.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub mccmnc: Option<String>,
    /// Network connection type. Refer to List: Connection Types in AdCOM 1.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::ConnectionType>>"))]
    pub connectiontype: Option<adcom::enums::ConnectionType>,
    /// ID sanctioned for advertiser use in the clear (i.e., not hashed).
    /// Unless prior arrangements have been made between the buyer and the
    /// seller directly, the value in this field is expected to be an ID
    /// derived from a call to an advertising API provided by the device's
    /// Operating System.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ifa: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub didsha1: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub didmd5: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub dpidsha1: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub dpidmd5: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub macsha1: Option<String>,
    /// Deprecated as of OpenRTB 2.6.
    #[deprecated(since="2.6.0")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub macmd5: Option<String>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
