use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateTask<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    
}