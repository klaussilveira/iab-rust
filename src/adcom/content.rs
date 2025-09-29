use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use super::enums::{CategoryTaxonomy, ProductionQuality, ContentContext, MediaRating};
use super::{data::Data, producer::Producer, network::Network, channel::Channel};

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64, AsEnum};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: Content
/// This object describes the content in which an impression can appear, which may be
/// syndicated or non-syndicated content. This object may be useful when syndicated
/// content contains impressions and does not necessarily match the publisher's general
/// content. An exchange may or may not have knowledge of the page where the content is
/// running as a result of the syndication method (e.g., a video impression embedded in
/// an iframe on an unknown web property or device).
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub struct Content {
    /// ID uniquely identifying the content.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub id: Option<String>,

    /// Episode number.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub episode: Option<i64>,

    /// Content title.
    /// Video Examples: "Search Committee" (television), "Star Wars, A New Hope" (movie),
    /// or "Endgame" (made for web).
    /// Non-Video Example: "Why an Antarctic Glacier Is Melting So Quickly" (Time magazine article).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub title: Option<String>,

    /// Content series.
    /// Video Examples: "The Office" (television), "Star Wars" (movie), or "Arby 'N' The Chief"
    /// (made for web).
    /// Non-Video Example: "Ecocentric" (Time Magazine blog).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub series: Option<String>,

    /// Content season (e.g., "Season 3").
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub season: Option<String>,

    /// Artist credited with the content.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub artist: Option<String>,

    /// Genre that best describes the content (e.g., rock, pop, etc).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub genre: Option<String>,

    /// Album to which the content belongs; typically for audio.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub album: Option<String>,

    /// International Standard Recording Code conforming to ISO-3901.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub isrc: Option<String>,

    /// A single URL of the content, for buy-side contextualization or review.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub url: Option<String>,

    /// Array of content categories describing the content using IDs from the taxonomy
    /// indicated in cattax. Implementer should ensure compliance with regional
    /// legislation around data usage and sharing.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub cat: Option<Vec<String>>,

    /// The taxonomy in use for the cat attribute. Refer to List: Category Taxonomies.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<CategoryTaxonomy>>"))]
    pub cattax: Option<CategoryTaxonomy>,

    /// Production quality. Refer to List: Production Qualities.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<ProductionQuality>>"))]
    pub prodq: Option<ProductionQuality>,

    /// Type of content (game, video, text, etc.). Refer to List: Content Contexts.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<ContentContext>>"))]
    pub context: Option<ContentContext>,

    /// Content rating (e.g., MPAA).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub rating: Option<String>,

    /// User rating of the content (e.g., number of stars, likes, etc.).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub urating: Option<String>,

    /// Media rating per IQG guidelines. Refer to List: Media Ratings.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsEnum<MediaRating>>"))]
    pub mrating: Option<MediaRating>,

    /// Comma separated list of keywords describing the content.
    /// This field is deprecated, use 'kwarray' instead.
    #[deprecated(note="This field is deprecated, use 'kwarray' instead")]
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub keywords: Option<String>,

    /// Array of keywords describing the content. Only one of 'keywords' or 'kwarray' may be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<Vec<AsString>>"))]
    pub kwarray: Option<Vec<String>>,

    /// Indication of live content, where 0=not live, 1=live (e.g., stream, live blog).
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub live: Option<i64>,

    /// Source relationship, where 0=indirect, 1=direct.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub srcrel: Option<i64>,

    /// Length of content in seconds; typically for video or audio.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub len: Option<i64>,

    /// Content language using ISO-639-1-alpha-2. Only one of lang or langb should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub lang: Option<String>,

    /// Content language using IETF BCP 47. Only one of lang or langb should be present.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub langb: Option<String>,

    /// Indicator of whether or not the content is embedded off-site from the the site
    /// or app described in those objects (e.g., an embedded video player), where
    /// 0=no, 1=yes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub embed: Option<i64>,

    /// Details about the content producer. Refer to Object: Producer.
    pub producer: Option<Producer>,

    /// Details about the network. Refer to Object: Network.
    pub network: Option<Network>,

    /// Details about the channel. Refer to Object: Channel.
    pub channel: Option<Channel>,

    /// Additional user data. Each Data object represents a different data source.
    /// Refer to Object: Data.
    pub data: Option<Vec<Data>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
