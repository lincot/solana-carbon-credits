use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Token};

#[derive(Accounts)]
pub struct MintCC<'info> {
    /// CHECK:
    #[account(seeds = [b"program_as_signer"], bump)]
    program_as_signer: UncheckedAccount<'info>,
    // TODO: check address
    authority: Signer<'info>,
    /// CHECK: only used in CPI
    #[account(mut, seeds = [b"cc_mint"], bump)]
    cc_mint: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut, seeds = [b"cc_reserve"], bump)]
    cc_reserve: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
}

pub fn mint_cc(ctx: Context<MintCC>, amount: u64, registry_batch_uri: String) -> Result<()> {
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.cc_mint.to_account_info(),
                to: ctx.accounts.cc_reserve.to_account_info(),
                authority: ctx.accounts.program_as_signer.to_account_info(),
            },
            &[&[
                b"program_as_signer",
                &[*ctx.bumps.get("program_as_signer").unwrap()],
            ]],
        ),
        amount * 10u64.pow(CC_DECIMALS as u32),
    )?;

    emit!(MintCCEvent {
        amount,
        registry_batch_uri,
    });

    Ok(())
}

#[event]
struct MintCCEvent {
    amount: u64,
    registry_batch_uri: String,
}
