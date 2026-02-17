use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub enum TaskStatus {
    Active,
    Completed,
    PunishmentPending,
    PunishmentCompleted,
}

pub struct Task {
    pub user: Pubkey,
    pub difficulty: Difficulty,
    pub stake_amount: u64,
    pub deadline: i64,
    pub status: TaskStatus,
    pub punishment_unlock_time: i64,
    pub bump: u8,
}