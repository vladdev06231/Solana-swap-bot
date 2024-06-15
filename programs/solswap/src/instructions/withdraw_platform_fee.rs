use crate::{constants::*, states::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  program::{invoke, invoke_signed},
  system_instruction,
};
use anchor_spl::{
  token::{self, Mint, Token, TokenAccount, Transfer},
};


#[derive(Accounts)]
pub struct WithdrawPlatformFee<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
      seeds = [SETTINGS_SEED],
      bump,
      has_one = admin,
    )]
    pub settings: Box<Account<'info, Settings>>,

    #[account(
      mut,
      seeds = [POOL_SEED],
      bump
    )]
    /// CHECK:
    pub pool: AccountInfo<'info>, 

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<WithdrawPlatformFee>) -> Result<()> {
    let accts = ctx.accounts;
    let signer_seeds: &[&[&[u8]]] = &[&[POOL_SEED.as_ref(), &[*ctx.bumps.get("pool").unwrap()]]];

    invoke_signed(
      &system_instruction::transfer(&accts.pool.key(), &accts.admin.key(), accts.pool.lamports() - Rent::get()?.minimum_balance(0)),
      &[
          accts.pool.to_account_info(),
          accts.admin.to_account_info(),
          accts.system_program.to_account_info(),
      ],
      signer_seeds,
    )?;
    
    Ok(())
}
