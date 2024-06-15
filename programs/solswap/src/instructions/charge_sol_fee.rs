// libraries
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  rent::Rent,
  program::{invoke, invoke_signed},
  system_instruction,
};

use crate::{constants::*, states::*, error::*};

pub fn handler(
    ctx: Context<ChargeSolFee>, 
    swap_amount: u64,
    // tokenamount_per_sol: u64,
    // slippage_bips: u64,
    platform_fee_bips: u64
) -> Result<()> {
    let accts = ctx.accounts;

    if !accts.botrole.addresses.contains(&accts.authority.key()) {
      return Err(CustomError::NotAllowedAuthority.into());
    }

    let gas_fee = 5000;
    // let mut real_swap_amount = swap_amount.checked_sub(gas_fee).unwrap();

    let fee = platform_fee_bips
      .checked_mul(swap_amount)
      .unwrap()
      .checked_div(10000)
      .unwrap()
      .checked_add(gas_fee)
      .unwrap();

    invoke(
      &system_instruction::transfer(&accts.authority.key(), &accts.pool.key(), fee),
      &[
          accts.authority.to_account_info(),
          accts.pool.to_account_info(),
          accts.system_program.to_account_info(),
      ],
    )?;
 
    Ok(())
}

#[derive(Accounts)]
pub struct ChargeSolFee<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
    )]
    pub botrole: Account<'info, BotRole>,

    #[account(
      mut,
      seeds = [POOL_SEED],
      bump,
    )]
    /// CHECK:
    pub pool: AccountInfo<'info>, 

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}