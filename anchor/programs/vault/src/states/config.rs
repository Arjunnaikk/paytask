use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub easy_price: u64,
    pub medium_price: u64,
    pub hard_price: u64,
    pub bump: u8,
}
