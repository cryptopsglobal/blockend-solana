use anchor_lang::prelude::*;
use anchor_spl::token::{Token, MintTo, mint_to};

declare_id!("5kDHrYTjDZ19d8jdj1RoVjomjLoAYK2XQPeBfZdFFMFL");

#[program]
pub mod mixarcade_solana {
  use super::*;

  pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
    let cpi_accounts = MintTo {
      mint: ctx.accounts.mint.to_account_info(),
      to: ctx.accounts.token_account.to_account_info(),
      authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // Create the cpi context
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    mint_to(cpi_ctx, 10)?;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
  #[account(mut)]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub mint: UncheckedAccount<'info>,
  pub token_program: Program<'info, Token>,
  #[account(mut)]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_account: UncheckedAccount<'info>,
  #[account(mut)]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub payer: AccountInfo<'info>
}
