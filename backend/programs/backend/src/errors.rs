use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid deadline")]
    InvalidDeadline,

    #[msg("Task already completed or invalid state")]
    InvalidState,

    #[msg("Deadline has passed")]
    DeadlinePassed,

    #[msg("Task not yet expired")]
    NotExpired,

    #[msg("Punishment period not over")]
    PunishmentNotOver,

    #[msg("Title exceeds maximum length of 64 characters")]
    TitleTooLong,

    #[msg("Description exceeds maximum length of 256 characters")]
    DescriptionTooLong,

    #[msg("Insufficient funds in vault")]
    InsufficientFunds,

    #[msg("Math operation overflow")]
    MathOverflow,   

    

}
