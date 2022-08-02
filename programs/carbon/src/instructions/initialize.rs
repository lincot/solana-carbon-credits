use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<ProgramState>(),
        seeds = [b"program_state"],
        bump,
    )]
    program_state: AccountLoader<'info, ProgramState>,
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [b"cc_mint"],
        bump,
        mint::authority = program_state,
        mint::decimals = CC_DECIMALS,
    )]
    cc_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        seeds = [b"cc_reserve"],
        bump,
        token::authority = program_state,
        token::mint = cc_mint,
    )]
    cc_reserve: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        seeds = [b"whitelist_mint"],
        bump,
        mint::authority = program_state,
        mint::freeze_authority = program_state,
        mint::decimals = 0,
    )]
    whitelist_mint: Account<'info, Mint>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state.load_init()?;
    program_state.bump = *ctx.bumps.get("program_state").unwrap();
    program_state.authority = ctx.accounts.authority.key();

    Ok(())
}
