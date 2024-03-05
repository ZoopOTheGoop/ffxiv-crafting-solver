use serde::{Deserialize, Serialize};

pub mod collectables;

/// A raw deserialized line from the `Recipe.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawRecipe {
    #[serde(rename = "#")]
    table_idx: u32,
    #[serde(rename = "Number")]
    number: u32,

    #[serde(rename = "CraftType")]
    craft_type: u32,

    #[serde(rename = "RecipeLevelTable")]
    rlvl: u32,

    #[serde(rename = "Item{Result}")]
    result_id: u32,

    #[serde(rename = "Amount{Result}")]
    result_quantity: u32,

    #[serde(rename = "Item{Ingredient}[0]")]
    ingredient_id_0: u32,
    #[serde(rename = "Amount{Ingredient}[0]")]
    ingredient_quantity_0: u32,

    #[serde(rename = "Item{Ingredient}[1]")]
    ingredient_id_1: u32,
    #[serde(rename = "Amount{Ingredient}[1]")]
    ingredient_quantity_1: u32,

    #[serde(rename = "Item{Ingredient}[2]")]
    ingredient_id_2: u32,
    #[serde(rename = "Amount{Ingredient}[2]")]
    ingredient_quantity_2: u32,

    #[serde(rename = "Item{Ingredient}[3]")]
    ingredient_id_3: u32,
    #[serde(rename = "Amount{Ingredient}[3]")]
    ingredient_quantity_3: u32,

    #[serde(rename = "Item{Ingredient}[4]")]
    ingredient_id_4: u32,
    #[serde(rename = "Amount{Ingredient}[4]")]
    ingredient_quantity_4: u32,

    #[serde(rename = "Item{Ingredient}[5]")]
    ingredient_id_5: u32,
    #[serde(rename = "Amount{Ingredient}[5]")]
    ingredient_quantity_5: u32,

    #[serde(rename = "Item{Ingredient}[6]")]
    ingredient_id_6: u32,
    #[serde(rename = "Amount{Ingredient}[6]")]
    ingredient_quantity_6: u32,

    #[serde(rename = "Item{Ingredient}[7]")]
    ingredient_id_7: u32,
    #[serde(rename = "Amount{Ingredient}[7]")]
    ingredient_quantity_7: u32,

    #[serde(rename = "Item{Ingredient}[8]")]
    ingredient_id_8: u32,
    #[serde(rename = "Amount{Ingredient}[8]")]
    ingredient_quantity_8: u32,

    #[serde(rename = "Item{Ingredient}[9]")]
    ingredient_id_9: u32,
    #[serde(rename = "Amount{Ingredient}[9]")]
    ingredient_quantity_9: u32,

    #[serde(rename = "RecipeNotebookList")]
    recipe_notebook_list: u32,

    #[serde(rename = "IsSecondary")]
    is_secondary: bool,

    #[serde(rename = "MaterialQualityFactor")]
    material_quality_factor: u8,

    #[serde(rename = "DifficultyFactor")]
    difficulty_factor: u16,

    #[serde(rename = "QualityFactor")]
    quality_factor: u16,

    #[serde(rename = "DurabilityFactor")]
    durability_factor: u16,

    /*

    Deleted record "" goes here, likely former "ProgressFactor"

    */
    #[serde(rename = "RequiredQuality")]
    required_quality: u32,

    #[serde(rename = "RequiredCraftsmanship")]
    required_craftsmanship: u16,

    #[serde(rename = "RequiredControl")]
    required_control: u16,

    #[serde(rename = "QuickSynthCraftsmanship")]
    quick_synth_craftsmanship: u16,

    #[serde(rename = "QuickSynthControl")]
    quick_synth_control: u16,

    #[serde(rename = "SecretRecipeBook")]
    secret_recipe_book: usize,

    #[serde(rename = "Quest")]
    quest: usize,

    #[serde(rename = "CanQuickSynth")]
    can_quick_synth: bool,

    #[serde(rename = "CanHq")]
    can_hq: bool,

    #[serde(rename = "ExpRewarded")]
    exp_rewarded: bool,

    #[serde(rename = "Status{Required}")]
    status_required: usize,

    #[serde(rename = "Item{Required}")]
    item_required: usize,

    #[serde(rename = "IsSpecializationRequired")]
    specialization_required: usize,

    #[serde(rename = "IsExpert")]
    is_expert: bool,

    /*

    Second deleted record "" goes here, no theory as to what it was

    */
    #[serde(rename = "PatchNumber")]
    patch_number: u16,
}

/// A raw deserialized line from the `RecipeLevelTable.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecipeLevel {
    #[serde(rename = "#")]
    rlvl: usize,

    #[serde(rename = "ClassJobLevel")]
    class_level: u8,

    #[serde(rename = "Stars")]
    stars: u8,

    #[serde(rename = "SuggestedCraftsmanship")]
    suggested_craftsmanship: u16,

    #[serde(rename = "SuggestedControl")]
    suggested_control: u16,

    #[serde(rename = "Difficulty")]
    difficulty: u16,
    #[serde(rename = "Quality")]
    quality: u32,

    #[serde(rename = "ProgressDivider")]
    progress_divider: u8,
    #[serde(rename = "QualityDivider")]
    quality_divider: u8,

    #[serde(rename = "ProgressModifier")]
    progress_modifier: u8,
    #[serde(rename = "QualityModifier")]
    quality_modifier: u8,

    #[serde(rename = "Durability")]
    durability: u8,

    #[serde(rename = "ConditionsFlag")]
    conditions_flag: u16,
}

/// A raw deserialized line from the `Items.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct RawItem {
    #[serde(rename = "#")]
    table_idx: usize,

    #[serde(rename = "Singular")]
    singular: String,
    #[serde(rename = "Adjective")]
    adjective: i8,
    #[serde(rename = "Plural")]
    plural: String,
    #[serde(rename = "PossessivePronoun")]
    possessive_pronoun: i8,
    #[serde(rename = "StartsWithVowel")]
    starts_with_vowel: i8,
    /*

    Deleted record "" goes here, no theory as to what it was

    */
    #[serde(rename = "Pronoun")]
    pronoun: i8,
    #[serde(rename = "Article")]
    article: i8,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Icon")]
    icon: u32,

    #[serde(rename = "Level{Item}")]
    ilvl: u32,
    #[serde(rename = "Rarity")]
    rarity: u8,

    #[serde(rename = "FilterGroup")]
    filter_group: u8,
    #[serde(rename = "AdditionalData")]
    additional_data: u32,

    #[serde(rename = "ItemUICategory")]
    item_ui_category: u32,
    #[serde(rename = "ItemSearchCategory")]
    item_search_category: u32,
    #[serde(rename = "EquipSlotCategory")]
    equip_slot_category: u32,
    #[serde(rename = "ItemSortCategory")]
    item_sort_category: u32,

    /*

    Deleted record "" goes here, no theory as to what it was

    */
    #[serde(rename = "StackSize")]
    stack_size: u32,

    #[serde(rename = "IsUnique")]
    is_unique: bool,
    #[serde(rename = "IsUntradable")]
    is_untradable: bool,
    #[serde(rename = "IsIndisposable")]
    is_indisposable: bool,
    #[serde(rename = "Lot")]
    lot: bool,
    #[serde(rename = "Price{Mid}")]
    price_mid: u32,
    #[serde(rename = "Price{Low}")]
    price_low: u32,
    #[serde(rename = "CanBeHq")]
    can_be_hq: bool,
    #[serde(rename = "IsDyeable")]
    is_dyeable: bool,
    #[serde(rename = "IsCrestWorthy")]
    is_crest_worthy: bool,
    #[serde(rename = "ItemAction")]
    item_action: u32,

    #[serde(rename = "CastTime<s>")]
    cast_time: u8,
    #[serde(rename = "Cooldown<s>")]
    cooldown_seconds: u16,

    #[serde(rename = "ClassJob{Repair}")]
    classjob_repair: u32,
    #[serde(rename = "Item{Repair}")]
    item_repair: u32,
    #[serde(rename = "Item{Glamour}")]
    item_glamour: u32,

    #[serde(rename = "Desynth")]
    desynth: u16,

    #[serde(rename = "IsCollectable")]
    is_collectable: bool,
    #[serde(rename = "AlwaysCollectable")]
    always_collectable: bool,

    #[serde(rename = "AetherialReduce")]
    aetherial_reduce: u16,

    #[serde(rename = "Level{Equip}")]
    level_equip: u8,
    /*

    Deleted record "" goes here, no theory as to what it was

    */
    #[serde(rename = "EquipRestriction")]
    equip_restriction: u8,
    #[serde(rename = "ClassJobCategory")]
    classjob_category: u32,

    #[serde(rename = "GrandCompany")]
    grand_company: u32,
    #[serde(rename = "ItemSeries")]
    item_series: u32,

    #[serde(rename = "BaseParamModifier")]
    base_param_modifier: u8,

    #[serde(rename = "Model{Main}")]
    model_main: i64,
    #[serde(rename = "Model{Sub}")]
    model_sub: i64,
    #[serde(rename = "ClassJob{Use}")]
    classjob_use: u32,
    /*

    Deleted record "" goes here, no theory as to what it was

    */
    #[serde(rename = "Damage{Phys}")]
    damage_phys: u16,
    #[serde(rename = "Damage{Mag}")]
    damage_mag: u16,
    #[serde(rename = "Delay<ms>")]
    delay_ms: u16,
    /*

    Deleted record "" goes here, no theory as to what it was

    */
    #[serde(rename = "BlockRate")]
    block_rate: u16,
    #[serde(rename = "Block")]
    block: u16,

    #[serde(rename = "Defense{Phys}")]
    defense_phys: u16,
    #[serde(rename = "Defense{Mag}")]
    defense_mag: u16,

    #[serde(rename = "BaseParam[0]")]
    base_param_0: u32,
    #[serde(rename = "BaseParamValue[0]")]
    base_param_value_0: i16,
    #[serde(rename = "BaseParam[1]")]
    base_param_1: u32,
    #[serde(rename = "BaseParamValue[1]")]
    base_param_value_1: i16,
    #[serde(rename = "BaseParam[2]")]
    base_param_2: u32,
    #[serde(rename = "BaseParamValue[2]")]
    base_param_value_2: i16,
    #[serde(rename = "BaseParam[3]")]
    base_param_3: u32,
    #[serde(rename = "BaseParamValue[3]")]
    base_param_value_3: i16,
    #[serde(rename = "BaseParam[4]")]
    base_param_4: u32,
    #[serde(rename = "BaseParamValue[4]")]
    base_param_value_4: i16,
    #[serde(rename = "BaseParam[5]")]
    base_param_5: u32,
    #[serde(rename = "BaseParamValue[5]")]
    base_param_value_5: i16,

    #[serde(rename = "ItemSpecialBonus")]
    item_special_bonus: u32,
    #[serde(rename = "ItemSpecialBonus{Param}")]
    item_special_bonus_param: u8,

    #[serde(rename = "BaseParam{Special}[0]")]
    base_param_special_0: u32,
    #[serde(rename = "BaseParamValue{Special}[0]")]
    base_param_value_special_0: i16,
    #[serde(rename = "BaseParam{Special}[1]")]
    base_param_special_1: u32,
    #[serde(rename = "BaseParamValue{Special}[1]")]
    base_param_value_special_1: i16,
    #[serde(rename = "BaseParam{Special}[2]")]
    base_param_special_2: u32,
    #[serde(rename = "BaseParamValue{Special}[2]")]
    base_param_value_special_2: i16,
    #[serde(rename = "BaseParam{Special}[3]")]
    base_param_special_3: u32,
    #[serde(rename = "BaseParamValue{Special}[3]")]
    base_param_value_special_3: i16,
    #[serde(rename = "BaseParam{Special}[4]")]
    base_param_special_4: u32,
    #[serde(rename = "BaseParamValue{Special}[4]")]
    base_param_value_special_4: i16,
    #[serde(rename = "BaseParam{Special}[5]")]
    base_param_special_5: u32,
    #[serde(rename = "BaseParamValue{Special}[5]")]
    base_param_value_special_5: i16,

    #[serde(rename = "MaterializeType")]
    materialize_type: u8,
    #[serde(rename = "MateriaSlotCount")]
    materia_slot_count: u8,
    #[serde(rename = "IsAdvancedMeldingPermitted")]
    advanced_melding_permitted: bool,
    #[serde(rename = "IsPvP")]
    is_pvp: bool,
    #[serde(rename = "SubStatCategory")]
    sub_stat_category: u8,
    #[serde(rename = "IsGlamourous")]
    is_glamourous: u8,
}
