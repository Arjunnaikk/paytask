use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod utils;
pub mod errors;
pub mod constants;

use instructions::*;

#[cfg(test)]
mod tests;

declare_id!("84qBhjkTzwDJeSnrmRLVGVsLFyjWe7ov7UNvJd6hvgP6");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        easy: u64,
        medium: u64,
        hard: u64,
    ) -> Result<()> {
        ctx.accounts.init_config(easy, medium, hard)
    }

    // pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
    //     create_profile::handler(ctx)
    // }

    // pub fn create_task(
    //     ctx: Context<CreateTask>,
    //     difficulty: Difficulty,
    //     deadline: i64,
    // ) -> Result<()> {
    //     create_task::handler(ctx, difficulty, deadline)
    // }

    // pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
    //     complete_task::handler(ctx)
    // }

    // pub fn mark_failed(ctx: Context<MarkFailed>) -> Result<()> {
    //     mark_failed::handler(ctx)
    // }

    // pub fn complete_punishment(ctx: Context<CompletePunishment>) -> Result<()> {
    //     complete_punishment::handler(ctx)
    // }
}