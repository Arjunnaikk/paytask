use anchor_lang::prelude::*;
use crate::{states::*, errors::ErrorCode, constants::*};

#[derive(Accounts)]
pub struct CreateTask<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile".as_ref(), user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,

    #[account(
        init,
        payer = user,
        space = Task::DISCRIMINATOR.len() + Task::INIT_SPACE,
        seeds = [b"task".as_ref(), user.key().as_ref(), profile.active_tasks.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), user.key().as_ref()],
        bump
    )]
    /// CHECK: vault only stores lamports
    pub vault: AccountInfo<'info>,

    #[account(
        seeds = [b"config".as_ref(), profile.user.as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTask<'info> {
    pub fn create_task(
        &mut self,
        title: String,
        description: String,
        difficulty: Difficulty,
        deadline: i64,
    ) -> Result<()> {

        let profile = &mut self.profile;
        let config = &self.config;
        let task = &mut self.task;

        let clock = Clock::get()?;
        let now = clock.unix_timestamp;

        require!(deadline > now, ErrorCode::InvalidDeadline);

        require!(title.len() <= 64, ErrorCode::TitleTooLong);
        require!(description.len() <= 256, ErrorCode::DescriptionTooLong);

        let base_price = match difficulty {
            Difficulty::Easy => config.easy_price,
            Difficulty::Medium => config.medium_price,
            Difficulty::Hard => config.hard_price,
        };

        let final_price = base_price
            .checked_mul(profile.stake_multiplier.into())
            .and_then(|result| result.checked_div(MULTIPLIER_BASE))
            .ok_or(ErrorCode::InvalidDeadline)?;

        // Transfer lamports
        **self.user.try_borrow_mut_lamports()? -= final_price;
        **self.vault.try_borrow_mut_lamports()? += final_price;

        self.task.set_inner(Task {
            user: self.user.key(),
            title,
            description,
            difficulty,
            stake_amount: final_price,
            deadline,
            punishment_unlock_time: 0,
            status: TaskStatus::Active,
            bump: self.task.bump,
        });

        profile.active_tasks += 1;

        Ok(())
    }
}