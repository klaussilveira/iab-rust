use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{display_format::DisplayFormat, native_format::NativeFormat, event_spec::EventSpec};
use super::enums::{
    PlacementPosition, ClickType, ApiFramework, CreativeSubtypeDisplay,
    SizeUnit, DisplayPlacementType, DisplayContextType,
};
use crate::defaults::{default_size_unit, default_clktype};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: DisplayPlacement
/// This object signals that the placement may be a display placement. It provides additional
/// detail about permitted display ads including simple banners, AMPHTML (i.e., Accelerated
/// Mobile Pages), and native.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct DisplayPlacement {
    /// Placement position on screen. Refer to List: Placement Positions.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<PlacementPosition>>"))]
    pub pos: Option<PlacementPosition>,

    /// Indicates if this is an interstitial placement, where 0=no, 1=yes.
    #[serde(default)]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub instl: i64,

    /// Indicates if the placement will be loaded into an iframe or not, where
    /// 0=unfriendly iframe or unknown, 1=top frame, friendly iframe, or SafeFrame.
    /// A value of "1" can be understood to mean that expandable ads are technically
    /// capable of being delivered.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub topframe: Option<i64>,

    /// Array of iframe busters supported by this placement. The meaning of strings
    /// in this attribute must be coordinated beforehand among vendors.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub ifrbust: Option<Vec<String>>,

    /// Indicates the click type of this placement. Refer to List: Click Types.
    #[serde(default="default_clktype")]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<ClickType>"))]
    pub clktype: ClickType,

    /// AMPHTML rendering treatment for AMP ads in this placement, where
    /// 1=early loading, 2=standard loading.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub ampren: Option<i64>,

    /// The display placement type. Refer to List: Display Placement Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<DisplayPlacementType>>"))]
    pub ptype: Option<DisplayPlacementType>,

    /// The context of the placement. Refer to List: Display Context Types.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<DisplayContextType>>"))]
    pub context: Option<DisplayContextType>,

    /// Array of supported mime types (e.g., "image/jpeg", "image/gif").
    /// If omitted, all types are assumed.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub mime: Option<Vec<String>>,

    /// List of supported APIs. If an API is not explicitly listed, it is assumed
    /// to be unsupported. Refer to List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ApiFramework>>>"))]
    pub api: Option<Vec<ApiFramework>>,

    /// Creative subtypes permitted. Refer to List: Creative Subtypes - Display.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<CreativeSubtypeDisplay>>>"))]
    pub ctype: Option<Vec<CreativeSubtypeDisplay>>,

    /// Width of the placement in units specified by unit. Note that this size
    /// applies to the placement itself; permitted creative sizes are specified
    /// elsewhere (e.g., DisplayFormat, ImageAssetFormat, etc.).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Height of the placement in units specified by unit. Note that this size
    /// applies to the placement itself; permitted creative sizes are specified
    /// elsewhere (e.g., DisplayFormat, ImageAssetFormat, etc.).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// Unit of size used for placement size (i.e., w and h attributes).
    /// Refer to List: Size Units.
    #[serde(default="default_size_unit")]
    #[cfg_attr(feature="coercion", serde_as(as="AsEnum<SizeUnit>"))]
    pub unit: SizeUnit,

    /// Indicator of whether or not the placement supports a buyer-specific privacy
    /// notice URL, where 0=no, 1=yes.
    #[serde(default)]
    #[serde(rename="priv")]
    #[cfg_attr(feature="coercion", serde_as(as="AsI64"))]
    pub r#priv: i64,

    /// Array of objects that govern the attributes (e.g., sizes) of a banner display
    /// placement. Refer to Object: DisplayFormat.
    pub displayfmt: Option<Vec<DisplayFormat>>,

    /// This object specified the required and permitted assets and attributes of a
    /// native display placement. Refer to Object: NativeFormat.
    pub nativefmt: Option<NativeFormat>,

    /// Array of supported ad tracking events. Refer to Object: EventSpec.
    pub event: Option<Vec<EventSpec>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
