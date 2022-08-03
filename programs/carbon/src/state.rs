use anchor_lang::prelude::*;

#[constant]
pub const CC_DECIMALS: u8 = 9;

#[account(zero_copy)]
#[repr(packed)]
pub struct ProgramState {
    pub bump: u8,
    pub authority: Pubkey,
}

#[derive(Copy, Clone, AnchorDeserialize, AnchorSerialize, Debug)]
pub enum CnftTier {
    Platinum,
    Gold,
    Silver,
    Bronze,
}

impl CnftTier {
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

    pub const fn metadata_uri(&self) -> &'static str {
        // TODO: use real hosted metadata
        match self {
            Self::Platinum => "https://arweave.net/a",
            Self::Gold => "https://arweave.net/b",
            Self::Silver => "https://arweave.net/c",
            Self::Bronze => "https://arweave.net/d",
        }
    }

    pub const fn collection_metadata_uri(&self) -> &'static str {
        // TODO: use real hosted metadata
        match self {
            Self::Platinum => "https://arweave.net/a",
            Self::Gold => "https://arweave.net/b",
            Self::Silver => "https://arweave.net/c",
            Self::Bronze => "https://arweave.net/d",
        }
    }
}

#[account(zero_copy)]
#[repr(packed)]
pub struct CnftData {
    pub creation_timestamp: u32,
    pub credits_per_year: u16,
    pub airdrops_claimed: u8,
}
