use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct UserProfile {
    pub user: Pubkey,
    pub active_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,

    pub success_streak: u32,
    pub failure_streak: u32,

    pub stake_multiplier: u64,

    pub bump: u8,
}