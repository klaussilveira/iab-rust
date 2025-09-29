use crate::adcom::enums::{CategoryTaxonomy, DoohVenueTaxonomy, PodSequence, SlotPositionInPod, UserAgentSource, AutoRefreshTrigger, ClickType, SizeUnit};

pub fn default_optional_i64_zero() -> Option<i64> {
    Some(0)
}

pub fn default_optional_i64_one() -> Option<i64> {
    Some(1)
}

pub fn default_optional_i64_two() -> Option<i64> {
    Some(2)
}

pub fn default_optional_f64_zero() -> Option<f64> {
    Some(0.0)
}

pub fn default_optional_string_usd() -> Option<String> {
    Some("USD".to_string())
}

pub fn default_optional_cattax_one() -> Option<CategoryTaxonomy> {
    Some(CategoryTaxonomy::IABTechLabContentCategoryTaxonomy1_0)
}

pub fn default_cattax_two() -> CategoryTaxonomy {
    CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_0
}

pub fn default_optional_cattax_nine() -> Option<CategoryTaxonomy> {
    Some(CategoryTaxonomy::IABTechLabContentTaxonomy3_1)
}

pub fn default_optional_venuetax_one() -> Option<DoohVenueTaxonomy> {
    Some(DoohVenueTaxonomy::OpenOOH1_0)
}

pub fn default_optional_podseq_zero() -> Option<PodSequence> {
    Some(PodSequence::AnyPosition)
}

pub fn default_optional_slotinpod_zero() -> Option<SlotPositionInPod> {
    Some(SlotPositionInPod::NotApplicable)
}

pub fn default_optional_uasource_zero() -> Option<UserAgentSource> {
    Some(UserAgentSource::Unknown(0))
}

pub fn default_optional_reftype_zero() -> Option<AutoRefreshTrigger> {
    Some(AutoRefreshTrigger::Unknown(0))
}

pub fn default_i64_one() -> i64 {
    1
}

pub fn default_size_unit() -> SizeUnit {
    SizeUnit::DeviceIndependentPixels
}

pub fn default_clktype() -> ClickType {
    ClickType::ClickableUnknown
}
