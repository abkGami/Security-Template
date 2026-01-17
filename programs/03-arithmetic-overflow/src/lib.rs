use anchor_lang::prelude::*;

pub mod vulnerable;
pub mod secure;

pub use vulnerable::*;
pub use secure::*;

declare_id!("5Z7MxiEHuVqN6xv5f7g3T4XFhQZnYSGGPHjFa8E5Rx6w");

#[program]
pub mod arithmetic_overflow {
    use super::*;

    // ========================================================================
    // VULNERABLE INSTRUCTIONS
    // ========================================================================
    
    pub fn deposit_insecure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
        vulnerable::deposit_insecure(ctx, amount)
    }
    
    pub fn withdraw_insecure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
        vulnerable::withdraw_insecure(ctx, amount)
    }
    
    pub fn calculate_rewards_insecure(ctx: Context<VaultOps>, multiplier: u64) -> Result<()> {
        vulnerable::calculate_rewards_insecure(ctx, multiplier)
    }
    
    // ========================================================================
    // SECURE INSTRUCTIONS
    // ========================================================================
    
    pub fn deposit_secure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
        secure::deposit_secure(ctx, amount)
    }
    
    pub fn withdraw_secure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
        secure::withdraw_secure(ctx, amount)
    }
    
    pub fn calculate_rewards_secure(ctx: Context<VaultOps>, multiplier: u64) -> Result<()> {
        secure::calculate_rewards_secure(ctx, multiplier)
    }
    
    pub fn compound_interest_secure(
        ctx: Context<VaultOps>,
        rate_numerator: u64,
        rate_denominator: u64,
        periods: u64,
    ) -> Result<()> {
        secure::compound_interest_secure(ctx, rate_numerator, rate_denominator, periods)
    }
    
    pub fn deposit_saturating(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
        secure::deposit_saturating(ctx, amount)
    }
}
