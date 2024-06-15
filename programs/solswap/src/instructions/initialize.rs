use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, states::*};

use anchor_lang::solana_program::{
  program::{invoke, invoke_signed},
  system_instruction,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [SETTINGS_SEED],
        bump,
        space = 8 + size_of::<Settings>()
    )]
    pub settings: Box<Account<'info, Settings>>,
    
    #[account(
      mut,
      seeds = [POOL_SEED],
      bump,
    )]
    /// CHECK:
    pub pool: AccountInfo<'info>, 

    #[account(
        init,
        payer = admin,
        seeds = [BOTROLE_SEED],
        bump,
        space = 8 + size_of::<Pubkey>() * 300
    )]
    pub botrole: Box<Account<'info, BotRole>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;
    let admin_key = accts.admin.key();
    accts.settings.admin = admin_key;
    accts.settings.pool_key = accts.pool.key();
    accts.botrole.addresses.push(admin_key);

    invoke(
      &system_instruction::transfer(&accts.admin.key(), &accts.pool.key(), Rent::get()?.minimum_balance(0)),
      &[
          accts.admin.to_account_info(),
          accts.pool.to_account_info(),
          accts.system_program.to_account_info(),
      ],
    )?;
    Ok(())
}
