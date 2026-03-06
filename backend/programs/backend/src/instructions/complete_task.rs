use anchor_lang::prelude::*;
use crate::{states::*, errors::ErrorCode, constants::*};

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile".as_ref(), user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [b"vault".as_ref(), user.key().as_ref()],
        bump
    )]
    /// CHECK: vault only stores lamports
    pub vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"task".as_ref(), user.key().as_ref(), profile.active_tasks.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,
}

impl<'info> CompleteTask<'info> {
    pub fn complete_task(&mut self) -> Result<()> {
        let task = &mut self.task;

    let now = Clock::get()?.unix_timestamp;

    require!(task.status == TaskStatus::Active, ErrorCode::InvalidState);
    require!(now <= task.deadline, ErrorCode::DeadlinePassed);

    **self.vault.try_borrow_mut_lamports()? -= task.stake_amount;
    **self.user.try_borrow_mut_lamports()? += task.stake_amount;

    task.status = TaskStatus::Completed;

    self.profile.set_inner(UserProfile {
        user: self.profile.user,
        active_tasks: self.profile.active_tasks - 1,
        completed_tasks: self.profile.completed_tasks + 1,
        failed_tasks: self.profile.failed_tasks,
        success_streak: self.profile.success_streak + 1,
        failure_streak: 0,
        stake_multiplier: MULTIPLIER_BASE,
        bump: self.profile.bump,
    });

    Ok(())
}

}