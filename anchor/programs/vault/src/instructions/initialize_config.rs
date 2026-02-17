use anchor_lang::prelude::*;

use crate::states::GlobalConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        // space = 8 + 32 + 8 * 3 + 1,
        space = GlobalConfig::DISCRIMINATOR.len() + GlobalConfig::INIT_SPACE,
        seeds = [b"config".as_ref(), authority.key().as_ref()],
        bump
    )]
    pub config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init_config(&mut self, easy: u64, medium: u64, hard: u64) -> Result<()> {
        // let config = &mut ctx.accounts.config;
        // config.authority = *ctx.accounts.authority.key;
        // config.easy_price = easy;
        // config.medium_price = medium;
        // config.hard_price = hard;
        // config.bump = ctx.bumps.config;

        self.config.set_inner(GlobalConfig {
            authority: *self.authority.key,
            easy_price: easy,
            medium_price: medium,
            hard_price: hard,
            bump: self.config.bump,
        });

        Ok(())
    }
}