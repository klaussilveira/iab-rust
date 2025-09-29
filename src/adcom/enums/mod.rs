/// Macro to implement Serialize and Deserialize for enums that convert to/from i64
#[macro_export]
macro_rules! impl_serde_for_enum {
    ($enum_type:ty) => {
        impl serde::Serialize for $enum_type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let value: i64=(*self).into();
                serializer.serialize_i64(value)
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value=i64::deserialize(deserializer)?;
                Ok(Self::from(value))
            }
        }
    };
}

// AdCOM 1.0 Enumerations
mod agent_type;
mod api_framework;
mod audit_status_code;
mod auto_refresh_trigger;
mod category_taxonomy;
mod click_type;
mod companion_type;
mod connection_type;
mod content_context;
mod creative_attribute;
mod creative_subtype_audio_video;
mod creative_subtype_display;
mod delivery_method;
mod device_type;
mod display_context_type;
mod display_placement_type;
mod dooh_multiplier_measurement_source_type;
mod dooh_venue_taxonomy;
mod dooh_venue_type;
mod event_tracking_method;
mod event_type;
mod expandable_direction;
mod feed_type;
mod id_match_method;
mod ip_location_service;
mod linearity_mode;
mod location_type;
mod media_rating;
mod native_data_asset_type;
mod native_image_asset_type;
mod operating_system;
mod playback_cessation_mode;
mod playback_method;
mod placement_position;
mod placement_subtype_video;
mod pod_deduplication;
mod pod_sequence;
mod production_quality;
mod size_unit;
mod slot_position_in_pod;
mod start_delay_mode;
mod user_agent_source;
mod volume_normalization_mode;

pub use agent_type::AgentType;
pub use api_framework::ApiFramework;
pub use audit_status_code::AuditStatusCode;
pub use auto_refresh_trigger::AutoRefreshTrigger;
pub use category_taxonomy::CategoryTaxonomy;
pub use click_type::ClickType;
pub use companion_type::CompanionType;
pub use connection_type::ConnectionType;
pub use content_context::ContentContext;
pub use creative_attribute::CreativeAttribute;
pub use creative_subtype_audio_video::CreativeSubtypeAudioVideo;
pub use creative_subtype_display::CreativeSubtypeDisplay;
pub use delivery_method::DeliveryMethod;
pub use device_type::DeviceType;
pub use display_context_type::DisplayContextType;
pub use display_placement_type::DisplayPlacementType;
pub use dooh_multiplier_measurement_source_type::DoohMultiplierMeasurementSourceType;
pub use dooh_venue_taxonomy::DoohVenueTaxonomy;
pub use dooh_venue_type::DoohVenueType;
pub use event_tracking_method::EventTrackingMethod;
pub use event_type::EventType;
pub use expandable_direction::ExpandableDirection;
pub use feed_type::FeedType;
pub use id_match_method::IdMatchMethod;
pub use ip_location_service::IpLocationService;
pub use linearity_mode::LinearityMode;
pub use location_type::LocationType;
pub use media_rating::MediaRating;
pub use native_data_asset_type::NativeDataAssetType;
pub use native_image_asset_type::NativeImageAssetType;
pub use operating_system::OperatingSystem;
pub use placement_position::PlacementPosition;
pub use placement_subtype_video::PlacementSubtypeVideo;
pub use playback_cessation_mode::PlaybackCessationMode;
pub use playback_method::PlaybackMethod;
pub use pod_deduplication::PodDeduplication;
pub use pod_sequence::PodSequence;
pub use production_quality::ProductionQuality;
pub use size_unit::SizeUnit;
pub use slot_position_in_pod::SlotPositionInPod;
pub use start_delay_mode::StartDelayMode;
pub use user_agent_source::UserAgentSource;
pub use volume_normalization_mode::VolumeNormalizationMode;
