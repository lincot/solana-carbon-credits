use anchor_lang::prelude::*;

pub const CC_DECIMALS: u8 = 9;

#[derive(Copy, Clone, AnchorDeserialize, AnchorSerialize, Debug)]
pub enum CNFTTier {
    Platinum,
    Gold,
    Silver,
    Bronze,
}
impl CNFTTier {
    pub const fn price(&self) -> u64 {
        match self {
            Self::Platinum => 20000,
            Self::Gold => 11000,
            Self::Silver => 3500,
            Self::Bronze => 1200,
        }
    }

    pub const fn credits_per_year(&self) -> u16 {
        match self {
            Self::Platinum => 200,
            Self::Gold => 100,
            Self::Silver => 30,
            Self::Bronze => 10,
        }
    }
}

#[account(zero_copy)]
#[repr(packed)]
pub struct CNFTData {
    pub creation_timestamp: u32,
    pub credits_per_year: u16,
    pub airdrops_claimed: u8,
}