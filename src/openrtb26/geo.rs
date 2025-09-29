use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::adcom;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Geo
/// This object encapsulates various methods for specifying a geographic location.
/// When subordinate to a `Device` object, it indicates the location of the device
/// which can also be interpreted as the user's current location. When subordinate
/// to a `User` object, it indicates the location of the user's home base (i.e.,
/// not necessarily their current location).
///
/// The `lat`/`lon` attributes should only be passed if they conform to the accuracy
/// depicted in the `type` attribute. For example, the centroid of a geographic
/// region such as postal code should not be passed.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Geo {
    /// Latitude from -90.0 to +90.0, where negative is south.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub lat: Option<f64>,
    /// Longitude from -180.0 to +180.0, where negative is west.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsF64>"))]
    pub lon: Option<f64>,
    /// Source of location data; recommended when passing `lat`/`lon`.
    /// Refer to List: Location Types in AdCOM 1.0.
    #[serde(rename="type")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::LocationType>>"))]
    pub type_: Option<adcom::enums::LocationType>,
    /// Estimated location accuracy in meters; recommended when `lat`/`lon`
    /// are specified and derived from a device's location services
    /// (i.e., type=1). Note that this is the accuracy as reported from
    /// the device. Consult OS specific documentation (e.g., Android, iOS)
    /// for exact interpretation.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub accuracy: Option<i64>,
    /// Number of seconds since this geolocation fix was established.
    /// Note that devices may cache location data across multiple fetches.
    /// Ideally, this value should be from the time the actual fix was taken.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub lastfix: Option<i64>,
    /// Service or provider used to determine geolocation from IP address
    /// if applicable (i.e., type=2). Refer to List: IP Location Services
    /// in AdCOM 1.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<adcom::enums::IpLocationService>>"))]
    pub ipservice: Option<adcom::enums::IpLocationService>,
    /// Country code using ISO-3166-1-alpha-3.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub country: Option<String>,
    /// Region code using ISO-3166-2; 2-letter state code if USA.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub region: Option<String>,
    /// Region of a country using FIPS 10-4 notation. While OpenRTB supports
    /// this attribute, it was withdrawn by NIST in 2008.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub regionfips104: Option<String>,
    /// Google metro code; similar to but not exactly Nielsen DMAs.
    /// See Appendix A for a link to the codes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub metro: Option<String>,
    /// City using United Nations Code for Trade & Transport Locations.
    /// See Appendix A for a link to the codes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub city: Option<String>,
    /// ZIP or postal code.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub zip: Option<String>,
    /// Local time as the number +/- of minutes from UTC.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub utcoffset: Option<i64>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
