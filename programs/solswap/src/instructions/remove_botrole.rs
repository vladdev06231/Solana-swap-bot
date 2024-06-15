use anchor_lang::prelude::*;
// use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct RemoveBotRole<'info> {
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

pub fn handler(ctx: Context<RemoveBotRole>, 
    address_to_remove: Pubkey,
) -> Result<()> {
  
    require!(address_to_remove.ne(&Pubkey::default()), CustomError::ZeroAddressDetected);
    let accts = ctx.accounts;
    if accts.botrole.addresses.contains(&address_to_remove) {
        let idx = accts.botrole.addresses.iter().position(|x| *x == address_to_remove).unwrap();
        accts.botrole.addresses.remove(idx);
    }
    Ok(())
}
