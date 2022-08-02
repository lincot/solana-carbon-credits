use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{freeze_account, mint_to, Token};

#[derive(Accounts)]
pub struct Whitelist<'info> {
    program_state: AccountLoader<'info, ProgramState>,
    #[account(address = program_state.load()?.authority)]
    authority: Signer<'info>,
    /// CHECK: only used in CPI
    #[account(mut, seeds = [b"whitelist_mint"], bump)]
    whitelist_mint: UncheckedAccount<'info>,
    /// CHECK: only used in CPI
    #[account(mut)]
    token_account: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
}

pub fn whitelist(ctx: Context<Whitelist>) -> Result<()> {
    let signers_seeds: &[&[&[u8]]] =
        &[&[b"program_state", &[ctx.accounts.program_state.load()?.bump]]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.whitelist_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            signers_seeds,
        ),
        1,
    )?;

    freeze_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::FreezeAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.whitelist_mint.to_account_info(),
            authority: ctx.accounts.program_state.to_account_info(),
        },
        signers_seeds,
    ))?;

    Ok(())
}
