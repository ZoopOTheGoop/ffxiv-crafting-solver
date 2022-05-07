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

/// Maps a condition to the raw modifier (before dividing by 100) from
/// the game files used in quality math.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityModifier {
    /// 50
    Poor = 50,
    /// 100
    Normal = 100,
    /// 150
    Good = 150,
    /// 400
    Excellent = 400,
}

/// Maps a condition to the raw modifier (before dividing by 100) from
/// the game files used in progress math.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProgressModifier {
    /// 150
    Malleable = 150,
    /// 100
    Normal = 100,
}

/// Maps a condition to the raw modifier from
/// the game files used in success rate math.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuccessRateModifier {
    /// 25
    Centered = 25,
    /// 0
    Normal = 0,
}

/// Maps a condition to the raw modifier (before dividing by 100) from
/// the game files used in durability math.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DurabilityModifier {
    /// 50
    Sturdy = 50,
    /// 100
    Normal = 100,
}

/// Maps a condition to the raw modifier from
/// the game files added on to status effects.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusDurationModifier {
    /// 2
    Primed = 2,
    /// 0
    Normal = 0,
}

/// Maps a condition to the raw modifier (before dividing by 100) from
/// the game files used in CP cost math.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpUsageModifier {
    /// 50
    Pliant = 50,
    /// 100
    Normal = 100,
}

// AKA 15
pub(crate) const NORMAL_CONDITIONS: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Excellent as u16
    | RawConditions::Poor as u16;

// AKA 115
pub(crate) const EXPERT_CRAFT_1: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Centered as u16
    | RawConditions::Pliant as u16
    | RawConditions::Sturdy as u16;

// AKA 483
pub(crate) const EXPERT_CRAFT_2: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Pliant as u16
    | RawConditions::Sturdy as u16
    | RawConditions::Malleable as u16
    | RawConditions::Primed as u16;

// AKA 499; corresponds only to RLVL 416 which I'm pretty sure isn't an actual in-use RLVL
pub(crate) const ALL_EXPERT_CONDITIONS_UNUSED: u16 = RawConditions::Normal as u16
    | RawConditions::Good as u16
    | RawConditions::Centered as u16
    | RawConditions::Pliant as u16
    | RawConditions::Sturdy as u16
    | RawConditions::Malleable as u16
    | RawConditions::Primed as u16;
