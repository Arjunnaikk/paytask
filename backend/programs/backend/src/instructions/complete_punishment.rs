use anchor_lang::prelude::*;

use crate::states::{Task, TaskStatus, UserProfile};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CompletePunishment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump,
        constraint = profile.user == user.key()
    )]
    pub profile: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [
            b"task",
            user.key().as_ref(),
            profile.active_tasks.to_le_bytes().as_ref()
        ],
        bump = task.bump,
        constraint = task.user == user.key()
    )]
    pub task: Account<'info, Task>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    /// CHECK: PDA holding lamports
    pub vault: AccountInfo<'info>,
}

impl<'info> CompletePunishment<'info> {
    pub fn complete_punishment(&mut self) -> Result<()> {
        let task = &mut self.task;

    let now = Clock::get()?.unix_timestamp;

    require!(
        task.status == TaskStatus::PunishmentPending,
        ErrorCode::InvalidState
    );

    require!(
        now >= task.punishment_unlock_time,
        ErrorCode::PunishmentNotOver
    );

    **self.vault.try_borrow_mut_lamports()? -= task.stake_amount;
    **self.user.try_borrow_mut_lamports()? += task.stake_amount;

    task.status = TaskStatus::PunishmentCompleted;

    Ok(())
}

}