use crate::{instructions::*, state::*};
use anchor_lang::prelude::*;

mod instructions;
pub mod state;
mod utils;

declare_id!("FLihr2MTD514e7hSHXAs2vu9t9i4KszsZ6SNz8jK6q1g");

#[program]
pub mod carbon {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn create_tier_collection(
        ctx: Context<CreateTierCollection>,
        tier: CNFTTier,
        metadata_uri: String,
    ) -> Result<()> {
        instructions::create_tier_collection(ctx, tier, metadata_uri)
    }

    pub fn whitelist(ctx: Context<Whitelist>) -> Result<()> {
        instructions::whitelist(ctx)
    }

    pub fn mint_cnft(ctx: Context<MintCNFT>, tier: CNFTTier) -> Result<()> {
        instructions::mint_cnft(ctx, tier)
    }

    pub fn mint_cc(ctx: Context<MintCC>, amount: u64, registry_batch_uri: String) -> Result<()> {
        instructions::mint_cc(ctx, amount, registry_batch_uri)
    }

    pub fn airdrop_cc(ctx: Context<AirdropCC>) -> Result<()> {
        instructions::airdrop_cc(ctx)
    }

    pub fn burn_cc(ctx: Context<BurnCC>, amount: u64) -> Result<()> {
        instructions::burn_cc(ctx, amount)
    }
}
