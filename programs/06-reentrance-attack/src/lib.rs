use anchor_lang::prelude::*;
pub mod vulnerable;
pub mod secure;
pub use vulnerable::*;
pub use secure::*;

declare_id!("BKm7nRJsPzQ3xH2fL9VTcW5Eg8YqX4NaZ6vD2MpF8kLx");

#[program]
pub mod reentrance_attack {
    use super::*;
    
    pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>, amount: u64) -> Result<()> {
        vulnerable::withdraw_vulnerable(ctx, amount)
    }
    
    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        secure::withdraw_secure(ctx, amount)
    }
}
