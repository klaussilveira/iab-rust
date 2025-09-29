use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{ApiFramework, CreativeSubtypeAudioVideo};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Audio
/// This object provides additional detail about an ad specifically for audio ads.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Audio {
    /// Mime type(s) of the ad creative(s) (e.g., "audio/mp4").
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub mime: Option<Vec<String>>,

    /// API required by the ad if applicable. Refer to List: API Frameworks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsEnum<ApiFramework>>>"))]
    pub api: Option<Vec<ApiFramework>>,

    /// Subtype of audio creative. Refer to List: Creative Subtypes - Audio/Video.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CreativeSubtypeAudioVideo>>"))]
    pub ctype: Option<CreativeSubtypeAudioVideo>,

    /// Duration of the audio creative in seconds.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub dur: Option<i64>,

    /// Audio markup (e.g., DAAST).
    /// Note that including both adm and curl is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub adm: Option<String>,

    /// Optional means of retrieving markup by reference; a URL that returns
    /// audio markup (e.g., DAAST). If this ad is matched to a Placement
    /// specification, the Placement.curlx attribute indicates if this markup
    /// retrieval option is supported.
    /// Note that including both adm and curl is not recommended.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub curl: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
