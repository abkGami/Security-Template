use anchor_lang::prelude::*;

pub mod vulnerable;
pub mod secure;

pub use vulnerable::*;
pub use secure::*;

declare_id!("8F1QcGh5RLKvZGJHxYFtN3TqZX2E8aVFfpwYL4NxH2Am");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
        vulnerable::withdraw_insecure(ctx, amount)
    }
    
    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        secure::withdraw_secure(ctx, amount)
    }
    
    pub fn initialize_user_stats(ctx: Context<InitializeUserStats>) -> Result<()> {
        secure::initialize_user_stats(ctx)
    }
}
