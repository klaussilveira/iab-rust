use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::{link_asset::LinkAsset, enums::VolumeNormalizationMode};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Banner
/// This object describes a basic banner creative. It is intended for display scenarios
/// that require a simple, structured image/link pair and is more secure than allowing
/// arbitrary HTML or JavaScript code.
///
/// Note: This implementation includes additional fields (w, h, vcm) beyond the base
/// AdCOM specification for compatibility with existing systems.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Banner {
    /// A URL that will return the image.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub img: String,

    /// Width of the creative in device independent pixels (DIPS).
    /// Note: This field is an extension beyond the base AdCOM specification.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub w: Option<i64>,

    /// Height of the creative in device independent pixels (DIPS).
    /// Note: This field is an extension beyond the base AdCOM specification.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub h: Option<i64>,

    /// Volume normalization mode for any video in the creative.
    /// Refer to List: Volume Normalization Modes.
    /// Note: This field is an extension beyond the base AdCOM specification.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<VolumeNormalizationMode>>"))]
    pub vcm: Option<VolumeNormalizationMode>,

    /// Destination link if the image is activated (e.g., clicked); not applicable
    /// in some contexts (e.g., DOOH) and its inclusion does not guarantee it will
    /// be supported. Refer to Object: LinkAsset.
    pub link: Option<LinkAsset>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
