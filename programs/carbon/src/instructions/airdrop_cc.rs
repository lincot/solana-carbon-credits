use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::clock::SECONDS_PER_DAY;

#[derive(Accounts)]
pub struct AirdropCC<'info> {
    program_state: AccountLoader<'info, ProgramState>,
    #[account(
        constraint = cnft_account.amount == 1,
        constraint = cnft_account.owner == cc_account.owner,
    )]
    cnft_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"cnft_data", cnft_account.mint.as_ref()],
        bump,
    )]
    cnft_data: AccountLoader<'info, CNFTData>,
    /// CHECK: only used in CPI
    #[account(mut, seeds = [b"cc_reserve"], bump)]
    cc_reserve: UncheckedAccount<'info>,
    #[account(mut)]
    cc_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}

pub fn airdrop_cc(ctx: Context<AirdropCC>) -> Result<()> {
    let cnft_data = &mut ctx.accounts.cnft_data.load_mut()?;

    let ts = Clock::get()?.unix_timestamp as u32;
    let airdrops_amount = 1 + (ts - cnft_data.creation_timestamp) / (365 * SECONDS_PER_DAY) as u32;
    let airdrops_amount = airdrops_amount.min(10) as u8 - cnft_data.airdrops_claimed;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.cc_reserve.to_account_info(),
                to: ctx.accounts.cc_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            &[&[b"program_state", &[ctx.accounts.program_state.load()?.bump]]],
        ),
        airdrops_amount as u64 * cnft_data.credits_per_year as u64 * 10u64.pow(CC_DECIMALS as u32),
    )?;

    cnft_data.airdrops_claimed += airdrops_amount;

    Ok(())
}
