use frame_support::weights::Weight;

/// Debug weight value for low weighted calls
pub const LOW: Weight = 5_000;

/// Debug weight value for medium weighted calls
pub const MID: Weight = 10_000;

/// Debug weight value for high weighted calls
pub const HIGH: Weight = 20_000;
