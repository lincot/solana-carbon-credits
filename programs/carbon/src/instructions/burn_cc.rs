use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct BurnCC<'info> {
    authority: Signer<'info>,
    /// CHECK: only used in CPI
    #[account(mut, seeds = [b"cc_mint"], bump)]
    cc_mint: UncheckedAccount<'info>,
    #[account(mut)]
    cc_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}

pub fn burn_cc(ctx: Context<BurnCC>, amount: u64) -> Result<()> {
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: ctx.accounts.cc_mint.to_account_info(),
                from: ctx.accounts.cc_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(BurnCCEvent {
        authority: ctx.accounts.authority.key(),
        amount,
    });

    Ok(())
}

#[event]
struct BurnCCEvent {
    authority: Pubkey,
    amount: u64,
}
