use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug, InitSpace)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug, InitSpace)]
pub enum TaskStatus {
    Active,
    Completed,
    PunishmentPending,
    PunishmentCompleted,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Task {
    pub user: Pubkey,
    #[max_len(50)]
    pub title: String,          // short text
    #[max_len(200)]
    pub description: String,    // longer text

    pub difficulty: Difficulty,
    pub stake_amount: u64,

    pub deadline: i64,                  // unix timestamp
    pub punishment_unlock_time: i64,    // unix timestamp when punishment can be executed

    pub status: TaskStatus,

    pub bump: u8,
}