use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct BotRole {
    pub addresses: Vec<Pubkey>,
}
