use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use solana_program::program::invoke_signed;

#[derive(Accounts)]
#[instruction(tier: CnftTier)]
pub struct CreateTierCollection<'info> {
    program_state: AccountLoader<'info, ProgramState>,
    #[account(mut, address = program_state.load()?.authority)]
    authority: Signer<'info>,
    /// CHECK: only used in CPI
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [b"tier_collection_mint", [tier as u8].as_ref()],
        bump,
        mint::authority = authority,
        mint::decimals = 0,
    )]
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = program_state,
    )]
    token_account: Account<'info, TokenAccount>,
    /// CHECK: only used in CPI
    #[account(mut)]
    edition: UncheckedAccount<'info>,
    /// CHECK: only used in CPI
    rent: UncheckedAccount<'info>,
    /// CHECK:
    #[account(address = mpl_token_metadata::ID)]
    token_metadata_program: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

pub fn create_tier_collection(ctx: Context<CreateTierCollection>, tier: CnftTier) -> Result<()> {
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    invoke_signed(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.program_state.key(),
            format!("{:?} Carbon NFT Collection", tier),
            format!("{:?}", tier),
            tier.collection_metadata_uri().into(),
            None,
            0,
            true,
            false,
            None,
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.program_state.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[b"program_state", &[ctx.accounts.program_state.load()?.bump]]],
    )?;

    invoke_signed(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.program_state.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.authority.key(),
            Some(0),
        ),
        &[
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.program_state.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[b"program_state", &[ctx.accounts.program_state.load()?.bump]]],
    )?;

    Ok(())
}
