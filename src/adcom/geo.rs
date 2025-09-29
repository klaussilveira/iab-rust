use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{LocationType, IpLocationService};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Geo
/// This object encapsulates various methods for specifying a geographic location.
/// When subordinate to a Device object, it indicates the location of the device
/// which can also be interpreted as the user's current location. When subordinate
/// to a User object, it indicates the location of the user's home base (i.e., not
/// necessarily their current location).
///
/// The lat and lon attributes should only be passed if they conform to the accuracy
/// depicted in the type attribute. For example, the centroid of a large region
/// (e.g., postal code) should not be passed.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Geo {
    /// Source of location data; recommended when passing lat/lon.
    /// Refer to List: Location Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<LocationType>>"))]
    pub r#type: Option<LocationType>,

    /// Latitude from -90.0 to +90.0, where negative is south.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub lat: Option<f64>,

    /// Longitude from -180.0 to +180.0, where negative is west.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub lon: Option<f64>,

    /// Estimated location accuracy in meters; recommended when lat/lon are
    /// specified and derived from a device's location services (i.e., type=1).
    /// Note that this is the accuracy as reported from the device. Consult OS
    /// specific documentation (e.g., Android, iOS) for exact interpretation.
    #[serde(alias="accuracy")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub accur: Option<i64>,

    /// Number of seconds since this geolocation fix was established. Note that
    /// devices may cache location data across multiple fetches. Ideally, this
    /// value should be from the time the actual fix was taken.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub lastfix: Option<i64>,

    /// Service or provider used to determine geolocation from IP address if
    /// applicable (i.e., type=2). Refer to List: IP Location Services.
    #[serde(alias="ipservice")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<IpLocationService>>"))]
    pub ipserv: Option<IpLocationService>,

    /// Country code using ISO-3166-1-alpha-2.
    /// Note that alpha-3 codes may be encountered and vendors are encouraged
    /// to be tolerant of them.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub country: Option<String>,

    /// Region code using ISO-3166-2; 2-letter state code if USA.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub region: Option<String>,

    /// Regional marketing areas such as Nielsen's DMA codes or other similar
    /// taxonomy to be agreed among vendors prior to use.
    /// Note that DMA is a trademarked asset of The Nielsen Company. Vendors
    /// are encouraged to ensure their use of DMAs is properly licensed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub metro: Option<String>,

    /// City using United Nations Code for Trade & Transport Locations "UN/LOCODE"
    /// with the space between country and city suppressed (e.g., Boston MA, USA="USBOS").
    /// Refer to UN/LOCODE Code List.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub city: Option<String>,

    /// ZIP or postal code.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub zip: Option<String>,

    /// Local time as the number +/- of minutes from UTC.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub utcoffset: Option<i64>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
