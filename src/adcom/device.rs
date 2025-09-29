use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::geo::Geo;
use super::user_agent::UserAgent;
use super::enums::{DeviceType, OperatingSystem, ConnectionType};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Device
/// This object provides information pertaining to the device through which the user is
/// interacting. Device information includes its hardware, platform, location, and carrier
/// data. The device can refer to a mobile handset, a desktop computer, set top box, or
/// other digital device.
///
/// Implementer should ensure compliance with regional legislation around data usage and sharing.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Device {
    /// The general type of device. Refer to List: Device Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<DeviceType>>"))]
    pub r#type: Option<DeviceType>,

    /// Browser user agent string. This field represents a raw user agent string
    /// from the browser. For backwards compatibility, exchanges are recommended
    /// to always populate ua with the User-Agent string, when available from the
    /// end user's device, even if an alternative representation, such as the
    /// User-Agent Client-Hints, is available and gets used to populate sua.
    /// No inferred or approximated user agents are expected in this field.
    /// If both ua and sua are present in the bid request, sua should be considered
    /// the more accurate representation of the device attributes. This is because
    /// the ua may contain a frozen or reduced UserAgent string.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ua: Option<String>,

    /// Structured user agent information defined by a Object: UserAgent.
    /// If both ua and sua are present in the bid request, sua should be
    /// considered the more accurate representation of the device attributes.
    /// This is because the ua may contain a frozen or reduced UserAgent string.
    pub sua: Option<UserAgent>,

    /// ID sanctioned for advertiser use in the clear (i.e., not hashed).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ifa: Option<String>,

    /// Standard "Do Not Track" flag as set in the header by the browser,
    /// where 0=tracking is unrestricted, 1=do not track.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub dnt: Option<i64>,

    /// "Limit Ad Tracking" signal commercially endorsed (e.g., iOS, Android),
    /// where 0=tracking is unrestricted, 1=tracking must be limited per
    /// commercial guidelines.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub lmt: Option<i64>,

    /// Device make (e.g., "Apple").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub make: Option<String>,

    /// Device model (e.g., "iPhone10,1" when the specific device model is known,
    /// "iPhone" otherwise). The value obtained from the device O/S should be
    /// used when available.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub model: Option<String>,

    /// Device operating system. Refer to List: Operating Systems.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<OperatingSystem>>"))]
    pub os: Option<OperatingSystem>,

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

    /// The ratio of physical pixels to device independent pixels.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub pxratio: Option<f64>,

    /// Support for JavaScript, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub js: Option<i64>,

    /// Browser language using ISO-639-1-alpha-2. Only one of lang or langb should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub lang: Option<String>,

    /// Browser language using IETF BCP 47. Only one of lang or langb should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub langb: Option<String>,

    /// IPv4 address closest to device.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ip: Option<String>,

    /// IP address closest to device as IPv6.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub ipv6: Option<String>,

    /// The value of the "x-forwarded-for" header.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub xff: Option<String>,

    /// Indicator of truncation of any of the IP attributes (i.e., ip, ipv6, xff),
    /// where 0=no, 1=yes (e.g., from 1.2.3.4 to 1.2.3.0).
    /// Refer to https://tools.ietf.org/html/rfc6235#section-4.1.1 for more
    /// information on IP truncation.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub iptr: Option<i64>,

    /// Carrier or ISP (e.g., "VERIZON") using exchange curated string names
    /// which should be published to bidders beforehand.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub carrier: Option<String>,

    /// Mobile carrier as the concatenated MCC-MNC code (e.g., "310-005" identifies
    /// Verizon Wireless CDMA in the USA). Refer to https://en.wikipedia.org/wiki/Mobile_country_code
    /// for further information and references. Note that the dash between the MCC
    /// and MNC parts is required to remove parsing ambiguity. The MCC-MNC values
    /// represent the SIM installed on the device and do not change when a device
    /// is roaming. Roaming may be inferred by a combination of the MCC-MNC, geo,
    /// IP and other data signals.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub mccmnc: Option<String>,

    /// MCC and MNC of the SIM card using the same format as mccmnc. When both
    /// values are available, a difference between them reveals that a user is roaming.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub mccmncsim: Option<String>,

    /// Network connection type. Refer to List: Connection Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<ConnectionType>>"))]
    pub contype: Option<ConnectionType>,

    /// Indicates if the geolocation API will be available to JavaScript code
    /// running in display ad, where 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub geofetch: Option<i64>,

    /// Location of the device (i.e., typically the user's current location).
    /// Refer to Object: Geo.
    pub geo: Option<Geo>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
