use anchor_lang::prelude::*;
// use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct AddBotRole<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
      seeds = [SETTINGS_SEED],
      bump,
      has_one = admin
    )]
    pub settings: Box<Account<'info, Settings>>,
    
    #[account(
        mut,
        seeds = [BOTROLE_SEED],
        bump
    )]
    pub botrole: Box<Account<'info, BotRole>>
}

pub fn handler(ctx: Context<AddBotRole>, 
    botrole_address: Pubkey,
) -> Result<()> {
    let accts = ctx.accounts;
    
    require!(botrole_address.ne(&Pubkey::default()), CustomError::ZeroAddressDetected);

    accts.botrole.addresses.push(botrole_address);
    Ok(())
}
