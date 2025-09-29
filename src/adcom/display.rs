use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{banner::Banner, native::Native, event::Event, enums::{ApiFramework, CreativeSubtypeDisplay}};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Display
/// This object provides additional detail about an ad specifically for display ads.
/// There are multiple attributes for specifying creative details: banner for simple
/// banner images, native for native ads, adm for including general markup, and curl
/// for referencing general markup via URL. In any given Display object, only one of
/// these attributes should be used to avoid confusion. To the extent feasible,
/// structured objects should be favored over general markup for quality and safety issues.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Display {
    /// Mime type of the ad (e.g., "image/jpeg").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub mime: Option<String>,

    /// API required by the ad if applicable. Refer to List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ApiFramework>>>"))]
    pub api: Option<Vec<ApiFramework>>,

    /// Subtype of display creative. Refer to List: Creative Subtypes - Display.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CreativeSubtypeDisplay>>"))]
    pub ctype: Option<CreativeSubtypeDisplay>,

    /// Absolute width of the creative in device independent pixels (DIPS),
    /// typically for non-native ads.
    /// Note that mixing absolute and relative sizes is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Absolute height of the creative in device independent pixels (DIPS),
    /// typically for non-native ads.
    /// Note that mixing absolute and relative sizes is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// Relative width of the creative when expressing size as a ratio,
    /// typically for non-native ads.
    /// Note that mixing absolute and relative sizes is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub wratio: Option<i64>,

    /// Relative height of the creative when expressing size as a ratio,
    /// typically for non-native ads.
    /// Note that mixing absolute and relative sizes is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub hratio: Option<i64>,

    /// URL of a page informing the user about a buyer's targeting activity.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub r#priv: Option<String>,

    /// General display markup (e.g., HTML, AMPHTML) if not using a structured
    /// alternative (e.g., banner, native).
    /// Note that including both adm and curl is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub adm: Option<String>,

    /// Optional means of retrieving display markup by reference; a URL that can
    /// return HTML, AMPHTML, or a collection native Asset object and their
    /// subordinates). If this ad is matched to a Placement specification, the
    /// Placement.curlx attribute indicates if this markup retrieval option is supported.
    /// Note that including both adm and curl is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub curl: Option<String>,

    /// Structured banner image object, recommended for simple banner creatives.
    /// Refer to Object: Banner.
    pub banner: Option<Banner>,

    /// Structured native object, recommended for native ads. Refer to Object: Native.
    pub native: Option<Native>,

    /// Array of events that the advertiser or buying platform wants to track.
    /// Refer to Object: Event.
    pub event: Option<Vec<Event>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
