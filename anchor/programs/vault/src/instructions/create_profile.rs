use anchor_lang::prelude::*;

use crate::states::UserProfile;

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserProfile::DISCRIMINATOR.len() + UserProfile::INIT_SPACE,
        seeds = [b"profile".as_ref(), user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,

     #[account(
        init,
        payer = user,
        seeds = [b"vault", user.key().as_ref()],
        bump,
        space = 8
    )]
    /// CHECK: vault only stores lamports
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProfile<'info> {
    pub fn create_profile(&mut self) -> Result<()> {
        self.profile.set_inner(UserProfile {
            user: *self.user.key,
            active_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            success_streak: 0,
            failure_streak: 0,
            stake_multiplier: 1,
            bump: self.profile.bump,
        });

        Ok(())
    }
}