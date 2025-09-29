#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// List: Category Taxonomies
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
pub enum CategoryTaxonomy {
    IABTechLabContentCategoryTaxonomy1_0,
    IABTechLabContentCategoryTaxonomy2_0,
    IABTechLabContentCategoryTaxonomy2_1,
    IABTechLabContentCategoryTaxonomy2_2,
    IABTechLabContentCategoryTaxonomy2_3,
    IABTechLabAdvertiserCategoryTaxonomy1_0,
    IABTechLabProductCategoryTaxonomy1_0,
    IABTechLabProductCategoryTaxonomy2_0,
    IABTechLabContentTaxonomy3_1,
    Unknown(i64),
}

impl From<i64> for CategoryTaxonomy {
    fn from(value: i64) -> Self {
        match value {
            1 => CategoryTaxonomy::IABTechLabContentCategoryTaxonomy1_0,
            2 => CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_0,
            3 => CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_1,
            4 => CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_2,
            5 => CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_3,
            6 => CategoryTaxonomy::IABTechLabAdvertiserCategoryTaxonomy1_0,
            7 => CategoryTaxonomy::IABTechLabProductCategoryTaxonomy1_0,
            8 => CategoryTaxonomy::IABTechLabProductCategoryTaxonomy2_0,
            9 => CategoryTaxonomy::IABTechLabContentTaxonomy3_1,
            _ => CategoryTaxonomy::Unknown(value),
        }
    }
}

impl From<CategoryTaxonomy> for i64 {
    fn from(value: CategoryTaxonomy) -> Self {
        match value {
            CategoryTaxonomy::IABTechLabContentCategoryTaxonomy1_0 => 1,
            CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_0 => 2,
            CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_1 => 3,
            CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_2 => 4,
            CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_3 => 5,
            CategoryTaxonomy::IABTechLabAdvertiserCategoryTaxonomy1_0 => 6,
            CategoryTaxonomy::IABTechLabProductCategoryTaxonomy1_0 => 7,
            CategoryTaxonomy::IABTechLabProductCategoryTaxonomy2_0 => 8,
            CategoryTaxonomy::IABTechLabContentTaxonomy3_1 => 9,
            CategoryTaxonomy::Unknown(v) => v,
        }
    }
}

crate::impl_serde_for_enum!(CategoryTaxonomy);
