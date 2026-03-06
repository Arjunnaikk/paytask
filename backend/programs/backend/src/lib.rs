use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;
pub mod constants;

use instructions::*;
use states::*;

declare_id!("EHhHvGkZqufszhfz1j46uQYKWEc5yw8BoxydpkmvfSUa");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        easy: u64,
        medium: u64,
        hard: u64,
    ) -> Result<()> {
        ctx.accounts.init_config(easy, medium, hard)
    }

    pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
        ctx.accounts.create_profile()
    }

    pub fn create_task(
        ctx: Context<CreateTask>,
        title: String,
        description: String,
        difficulty: Difficulty,
        deadline: i64,
    ) -> Result<()> {
        ctx.accounts.create_task(title, description, difficulty, deadline)
    }

    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        ctx.accounts.complete_task()
    }

    pub fn mark_failed(ctx: Context<MarkFailed>) -> Result<()> {
        ctx.accounts.mark_failed()
    }

    pub fn complete_punishment(ctx: Context<CompletePunishment>) -> Result<()> {
        ctx.accounts.complete_punishment()
    }
}






