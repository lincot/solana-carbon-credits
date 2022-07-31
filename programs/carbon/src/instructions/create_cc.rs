use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateCC<'info> {
    /// CHECK:
    #[account(seeds = [b"program_as_signer"], bump)]
    program_as_signer: UncheckedAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: only used in CPI
    #[account(
        init,
        payer = payer,
        seeds = [b"cc_mint"],
        bump,
        mint::authority = program_as_signer,
        mint::decimals = CC_DECIMALS,
    )]
    cc_mint: Account<'info, Mint>,
    /// CHECK:
    #[account(
        init,
        payer = payer,
        seeds = [b"cc_reserve"],
        bump,
        token::authority = program_as_signer,
        token::mint = cc_mint,
    )]
    cc_reserve: Account<'info, TokenAccount>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn create_cc(_ctx: Context<CreateCC>) -> Result<()> {
    Ok(())
}
