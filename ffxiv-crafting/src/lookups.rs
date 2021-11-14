#![allow(dead_code)]

// It's 101 because it goes from [0-100], not [1-100]
pub(crate) const HQ: [u8; 101] = [
    1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
    9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 17,
    17, 17, 18, 18, 18, 19, 19, 20, 20, 21, 22, 23, 24, 26, 28, 31, 34, 38, 42, 47, 52, 58, 64, 68,
    71, 74, 76, 78, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 94, 96, 98, 100,
];

pub(crate) const fn lookup_hq(quality: u32, recipe_quality: u32) -> u8 {
    // Compute integer percentage without casting -- this gives the same result as going to
    // float and then truncating from conversion
    let raw_chance = (quality * 200 + recipe_quality) / (recipe_quality * 2);

    // Can't use `min` in const functions :(
    HQ[if raw_chance > 100 { 100 } else { raw_chance } as usize]
}

pub(crate) const CLVL: [u16; 80] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    120, 125, 130, 133, 136, 139, 142, 145, 148, 150, 260, 265, 270, 273, 276, 279, 282, 285, 288,
    290, 390, 395, 400, 403, 406, 409, 412, 415, 418, 420,
];

pub(crate) const RLVL: [u16; 80 + 4 * 3 + 9] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    55, 70, 90, 110, 115, 125, 130, 133, 136, 139, 142, 145, 148, 150, 160, 180, 210, 220, 250,
    255, 265, 270, 273, 276, 279, 282, 285, 288, 290, 300, 320, 350, 380, 390, 395, 400, 403, 406,
    409, 412, 415, 418, 430, 440, 450, 480, 481, 490, 511, 512, 513,
];

pub(crate) const RLVL_CRAFTSMANSHIP: [u16; 80 + 4 * 3 + 9] = [
    22, 22, 22, 22, 50, 50, 50, 59, 59, 59, 67, 67, 67, 67, 67, 78, 78, 78, 82, 94, 94, 94, 99, 99,
    99, 99, 99, 106, 106, 106, 121, 121, 121, 129, 129, 129, 129, 129, 136, 136, 136, 150, 150,
    150, 150, 150, 161, 161, 161, 176, 325, 325, 391, 451, 468, 502, 519, 529, 539, 550, 560, 570,
    580, 587, 620, 718, 850, 870, 995, 1006, 1027, 1037, 1044, 1050, 1056, 1063, 1069, 1075, 1079,
    1100, 1320, 1500, 1650, 1320, 1388, 1457, 1498, 1539, 1580, 1621, 1662, 1702, 1866, 2000, 2140,
    2480, 2484, 2520, 2620, 2620, 2620,
];

pub(crate) const RLVL_CONTROL: [u16; 80 + 4 * 3 + 9] = [
    11, 11, 11, 11, 25, 25, 25, 29, 29, 29, 33, 33, 33, 33, 33, 39, 39, 39, 41, 47, 47, 47, 49, 49,
    49, 49, 49, 53, 53, 53, 60, 60, 60, 64, 64, 64, 64, 64, 68, 68, 68, 75, 75, 75, 75, 75, 80, 80,
    80, 88, 325, 325, 374, 407, 426, 462, 480, 491, 502, 513, 524, 535, 546, 553, 589, 695, 820,
    835, 955, 968, 993, 1005, 1013, 1020, 1028, 1035, 1043, 1050, 1055, 1080, 1220, 1350, 1600,
    1220, 1284, 1348, 1387, 1425, 1464, 1502, 1541, 1579, 1733, 1860, 1990, 2195, 2206, 2305, 2540,
    2540, 2540,
];

pub(crate) const RLVL_PROGRESS: [u32; 80 + 4 * 3 + 9] = [
    19, 20, 20, 21, 33, 36, 37, 41, 42, 45, 48, 53, 54, 54, 55, 63, 66, 67, 68, 74, 75, 75, 79, 85,
    89, 90, 91, 100, 101, 102, 106, 110, 111, 115, 123, 124, 128, 129, 137, 138, 143, 144, 155,
    156, 158, 159, 167, 172, 174, 186, 195, 233, 445, 586, 339, 503, 586, 641, 697, 752, 808, 863,
    919, 956, 982, 1033, 1106, 1234, 1476, 1116, 1263, 1476, 1586, 1697, 1808, 1919, 2029, 2140,
    2214, 2361, 2657, 2760, 2900, 3149, 3248, 3348, 3407, 3467, 3526, 3586, 3645, 3705, 3943, 4143,
    4343, 4943, 4963, 5143, 5563, 5583, 5603,
];

pub(crate) const RLVL_QUALITY: [u32; 80 + 4 * 3 + 9] = [
    312, 325, 339, 352, 451, 474, 492, 526, 545, 629, 665, 702, 726, 751, 807, 866, 898, 939, 982,
    1053, 1090, 1122, 1169, 1239, 1296, 1332, 1368, 1498, 1544, 1584, 1670, 1697, 1757, 1811, 1853,
    1882, 1905, 1961, 2026, 2050, 2109, 2147, 2251, 2277, 2309, 2372, 2421, 2524, 2551, 2641, 2646,
    2921, 4980, 5783, 3951, 5172, 5783, 6042, 6301, 6561, 6820, 7080, 7339, 7851, 7874, 8015, 8298,
    8742, 9230, 8377, 8581, 9186, 9657, 10023, 10389, 10755, 11121, 11490, 11736, 11960, 12511,
    13144, 14267, 13086, 13660, 14062, 14482, 14902, 15322, 15742, 16162, 16582, 18262, 19662,
    23395, 25863, 25945, 26686, 28414, 28496, 28578,
];

pub(crate) const RLVL_DURABILITY: [u8; 80 + 4 * 3 + 9] = [
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 80, 80, 80,
    80, 80, 80, 80, 80, 80, 80, 70, 70, 70, 70, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 70, 70, 70,
    70, 70, 70, 70, 70,
];

/* RLVL conditions are at the bottom because it's long */

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecipeLevelRanges {
    ArrLeveling(u8),
    ArrMax(Stars),
    HwLeveling(u8),
    HwMax(Stars),
    StbLeveling(u8),
    StbMax(Stars),
    ShbLeveling(u8),
    ShbMax(ShbStars),
}

impl RecipeLevelRanges {
    pub const fn to_canonical_idx(self) -> usize {
        (match self {
            Self::ArrLeveling(lvl)
            | Self::HwLeveling(lvl)
            | Self::StbLeveling(lvl)
            | Self::ShbLeveling(lvl) => {
                let modifier = (lvl - 40) / 10;
                // Four stars of recipes every 10 levels (until ShB max...)
                lvl + modifier * 4
            }
            Self::ArrMax(stars) => 50 + stars as u8,
            Self::HwMax(stars) => 60 + stars as u8,
            Self::StbMax(stars) => 70 + stars as u8,
            Self::ShbMax(stars) => 80 + stars as u8,
        }) as usize
    }

    pub const fn to_recipe_level(self) -> u16 {
        RLVL[self.to_canonical_idx()]
    }

    pub const fn to_recipe_level_craftsmanship(self) -> u16 {
        RLVL_CRAFTSMANSHIP[self.to_canonical_idx()]
    }

    pub const fn to_recipe_level_progress(self) -> u32 {
        RLVL_PROGRESS[self.to_canonical_idx()]
    }

    pub const fn to_recipe_level_control(self) -> u16 {
        RLVL_CONTROL[self.to_canonical_idx()]
    }

    pub const fn to_recipe_level_quality(self) -> u32 {
        RLVL_QUALITY[self.to_canonical_idx()]
    }

    pub const fn to_recipe_level_durability(self) -> u8 {
        RLVL_DURABILITY[self.to_canonical_idx()]
    }

    const fn to_recipe_level_conditions(self) -> ConditionBits {
        RLVL_CONDITIONS[self.to_canonical_idx()]
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stars {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShbStars {
    Zero = 0,
    One,
    Two,
    Three,
    ThreeGold,
    Four,
    FourGold,
    FourGold2,
    FourGold3,
}

pub(crate) enum RawConditions {
    Normal = 0x01,
    Good = 0x02,
    Excellent = 0x04,
    Poor = 0x08,
    Centered = 0x10,
    Pliant = 0x20,
    Sturdy = 0x40,
    Malleable = 0x80,
    Primed = 0x100,
}

/* Wanted to do these as one big enum but since Malleable has the same mod as Poor we can't */
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityModifier {
    Poor = 50,
    Normal = 100,
    Good = 150,
    Excellent = 400,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProgressModifier {
    Malleable = 150,
    Normal = 100,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuccessRateModifier {
    Centered = 25,
    Normal = 0,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DurabilityModifier {
    Sturdy = 50,
    Normal = 100,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusDurationModifier {
    Primed = 2,
    Normal = 0,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpUsageModifier {
    Pliant = 50,
    Normal = 0,
}

// This is largely here because it's how it is in the files, we use the
// enum conversions
pub(crate) const CONDITION_MODIFIER_TABLE: [u16; 5] = [
    QualityModifier::Poor as u16,
    QualityModifier::Normal as u16,
    QualityModifier::Good as u16,
    QualityModifier::Excellent as u16,
    ProgressModifier::Malleable as u16,
];

pub(crate) const NORMAL_CONDITIONS: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Excellent as u16
    | RawConditions::Poor as u16;

pub(crate) const EXPERT_CRAFT_1: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Centered as u16
    | RawConditions::Pliant as u16
    | RawConditions::Sturdy as u16;

pub(crate) const EXPERT_CRAFT_2: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Pliant as u16
    | RawConditions::Sturdy as u16
    | RawConditions::Malleable as u16
    | RawConditions::Primed as u16;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ConditionBits(u16);

pub(crate) const RLVL_CONDITIONS: [ConditionBits; 80 + 4 * 3 + 9] = [
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(EXPERT_CRAFT_1),
    ConditionBits(NORMAL_CONDITIONS),
    ConditionBits(EXPERT_CRAFT_1),
    ConditionBits(EXPERT_CRAFT_1),
    ConditionBits(EXPERT_CRAFT_2),
];

#[cfg(test)]
mod test {
    use super::RLVL_CONDITIONS;

    const RLVL_CONDITIONS_RAW: [u16; 80 + 4 * 3 + 9] = [
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 115, 15, 115, 115, 483,
    ];

    #[test]
    fn rlvl_matches_conditions() {
        assert_eq!(RLVL_CONDITIONS_RAW, RLVL_CONDITIONS.map(|v| v.0));
    }
}