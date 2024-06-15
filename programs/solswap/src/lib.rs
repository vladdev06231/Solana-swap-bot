use anchor_lang::prelude::*;

declare_id!("9fuV5utLoM5U81FQoiZLYB3moJzyJcV477cTpDYArmV1");

/// constant
pub mod constants;
/// error
pub mod error;
/// instructions
pub mod instructions;
/// states
pub mod states;

use crate::instructions::*;

#[program]
pub mod solswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn add_botrole(ctx: Context<AddBotRole>, addr: Pubkey) -> Result<()> {
      add_botrole::handler(ctx, addr)
    }

    pub fn remove_botrole(ctx: Context<RemoveBotRole>, addr: Pubkey) -> Result<()> {
      remove_botrole::handler(ctx, addr)
    }

    pub fn charge_sol_fee(ctx: Context<ChargeSolFee>,
      swap_amount: u64,
      // tokenamount_per_sol: u64,
      // slippage_bips: u64,
      platform_fee_bips: u64
    ) -> Result<()> {
      charge_sol_fee::handler(
        ctx,
        swap_amount,
        // tokenamount_per_sol,
        // slippage_bips,
        platform_fee_bips
      )
    }
    
    pub fn withdraw_platform_fee(ctx: Context<WithdrawPlatformFee>) -> Result<()> {
      withdraw_platform_fee::handler(ctx)
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}
