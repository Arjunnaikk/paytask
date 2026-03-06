use anchor_lang::prelude::*;

use crate::{states::*, errors::ErrorCode, constants::*};

#[derive(Accounts)]
pub struct MarkFailed<'info> {
    /// The original task owner
    pub user: SystemAccount<'info>,

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
}

impl<'info> MarkFailed<'info> {
    pub fn mark_failed(&mut self) -> Result<()> {
        let profile = &mut self.profile;
        let task = &mut self.task;

    let now = Clock::get()?.unix_timestamp;

    require!(task.status == TaskStatus::Active, ErrorCode::InvalidState);
    require!(now > task.deadline, ErrorCode::NotExpired);

    task.status = TaskStatus::PunishmentPending;
    task.punishment_unlock_time = now + PUNISHMENT_DELAY;

    profile.active_tasks -= 1;
    profile.failed_tasks += 1;

    profile.success_streak = 0;
    profile.failure_streak += 1;

    let new_multiplier = profile.stake_multiplier
        .checked_mul(MULTIPLIER_INCREASE)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(MULTIPLIER_BASE)
        .ok_or(ErrorCode::MathOverflow)?;

    profile.stake_multiplier = new_multiplier.min(MULTIPLIER_MAX);

    Ok(())
}

}