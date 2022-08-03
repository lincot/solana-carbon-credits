use crate::ID;
use crate::{state::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, mint_to, Mint, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v2, verify_collection},
    state::Collection,
};
use solana_program::program::invoke_signed;

#[derive(Accounts)]
#[instruction(tier: CnftTier)]
pub struct MintCnft<'info> {
    // TODO: remove mut when they fix unnecessary isMut
    #[account(mut)]
    program_state: AccountLoader<'info, ProgramState>,
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        token::authority = authority,
        token::mint = Pubkey::find_program_address(&[b"whitelist_mint"], &ID).0,
        constraint = authority_whitelist.amount != 0,
    )]
    authority_whitelist: Box<Account<'info, TokenAccount>>,
    /// CHECK: only used in CPI
    #[account(mut)]
    authority_usdc: UncheckedAccount<'info>,
    #[account(
        mut,
        token::authority = program_state.load()?.authority,
        token::mint = USDC_MINT,
    )]
    platform_usdc: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        mint::authority = authority,
        mint::decimals = 0,
    )]
    mint: Account<'info, Mint>,
    /// CHECK: only used in CPI
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: only used in CPI
    #[account(mut)]
    edition: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    token_account: Account<'info, TokenAccount>,
    /// CHECK: only used in CPI
    #[account(seeds = [b"tier_collection_mint", [tier as u8].as_ref()], bump)]
    collection_mint: UncheckedAccount<'info>,
    /// CHECK: only used in CPI
    collection_metadata: UncheckedAccount<'info>,
    /// CHECK: only used in CPI
    collection_edition: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<CnftData>(),
        seeds = [b"cnft_data", mint.key().as_ref()],
        bump,
    )]
    cnft_data: AccountLoader<'info, CnftData>,
    /// CHECK: only used in CPI
    rent: UncheckedAccount<'info>,
    /// CHECK:
    #[account(address = mpl_token_metadata::ID)]
    token_metadata_program: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

pub fn mint_cnft(ctx: Context<MintCnft>, tier: CnftTier) -> Result<()> {
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.authority_usdc.to_account_info(),
                to: ctx.accounts.platform_usdc.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        tier.price(),
    )?;

    mint_to(
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

    let signers_seeds: &[&[&[u8]]] =
        &[&[b"program_state", &[ctx.accounts.program_state.load()?.bump]]];

    invoke_signed(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.program_state.key(),
            format!("{:?} Carbon NFT", tier),
            format!("{:?}", tier),
            tier.metadata_uri().into(),
            None,
            0,
            true,
            false,
            Some(Collection {
                verified: false,
                key: ctx.accounts.collection_mint.key(),
            }),
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
        signers_seeds,
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
        signers_seeds,
    )?;

    invoke_signed(
        &verify_collection(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.program_state.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.collection_mint.key(),
            ctx.accounts.collection_metadata.key(),
            ctx.accounts.collection_edition.key(),
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.program_state.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_edition.to_account_info(),
        ],
        signers_seeds,
    )?;

    let cnft_data = &mut ctx.accounts.cnft_data.load_init()?;
    cnft_data.creation_timestamp = Clock::get()?.unix_timestamp as u32;
    cnft_data.credits_per_year = tier.credits_per_year();

    Ok(())
}
