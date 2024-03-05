//! Data row implementations from the `Collectables*.csv` series of tables.

use serde::{Deserialize, Serialize};

/// A raw deserialized line from the `CollectablesShopItem.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawCollectablesShopItem {
    #[serde(rename = "#")]
    table_idx: usize,

    #[serde(rename = "Item")]
    item_id: usize,

    #[serde(rename = "CollectablesShopItemGroup")]
    collectables_shop_item_group: usize,
    #[serde(rename = "LevelMin")]
    level_min: u16,
    #[serde(rename = "LevelMax")]
    level_max: u16,
    #[serde(rename = "Stars")]
    stars: u8,
    #[serde(rename = "Key")]
    key: u8,
    #[serde(rename = "CollectablesShopRefine")]
    collectables_shop_refine: usize,
    #[serde(rename = "CollectablesShopRewardScrip")]
    collectables_shop_reward_scrip: usize,
}

/// A raw deserialized line from the `CollectablesShopRewardItem.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawCollectablesShopRewardItem {
    #[serde(rename = "#")]
    table_idx: usize,

    #[serde(rename = "Item")]
    item_id: usize,
    /*
    Blank entry here
    */
    #[serde(rename = "RewardLow")]
    reward_low: u8,
    #[serde(rename = "RewardMid")]
    reward_mid: u16,
    #[serde(rename = "RewardHigh")]
    reward_high: u16,
    /*
    5 blank entries here
    */
}

/// A raw deserialized line from the `CollectablesShopRefine.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawCollectablesShopRefine {
    #[serde(rename = "#")]
    table_idx: usize,

    #[serde(rename = "LowCollectability")]
    low_collectability: u16,
    #[serde(rename = "MidCollectability")]
    mid_collectability: u16,
    #[serde(rename = "HighCollectability")]
    high_collectability: u16,
}

// Omitting CollectablesShopItemGroup because we don't need the names (right now)

/// A raw deserialized line from the `CollectablesShopItem.csv` file.
///
/// Most code will never interact with this, but through a friendlier struct that collects some of this info
/// in a more sane way.
#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CollectablesShopItem {
    // For some reason rather than a normal index this has a weird major.minor type format
    #[serde(rename = "#")]
    table_idx: String,

    #[serde(rename = "Item")]
    item_id: usize,
    #[serde(rename = "CollectablesShopItemGroup")]
    collectables_shop_item_group: usize,

    #[serde(rename = "LevelMin")]
    level_min: u16,
    #[serde(rename = "LevelMax")]
    level_max: u16,
    #[serde(rename = "Stars")]
    stars: u8,

    #[serde(rename = "Key")]
    key: u8,

    #[serde(rename = "CollectablesShopRefine")]
    collectables_shop_refine: usize,

    #[serde(rename = "CollectablesShopRewardScrip")]
    collectables_shop_reward_scrip: usize,
}
