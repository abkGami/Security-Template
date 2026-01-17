// ✅ SECURE - Anchor handles discriminator
use anchor_lang::prelude::*;

pub fn process_secure(ctx: Context<ProcessSecure>) -> Result<()> {
    // ✅ Account<'info, Config> verifies discriminator automatically
    let config = &ctx.accounts.config;
    
    require!(config.enabled, ErrorCode::ConfigDisabled);
    msg!("Processing with verified admin: {}", config.admin);
    Ok(())
}

#[derive(Accounts)]
pub struct ProcessSecure<'info> {
    /// ✅ Account type enforces discriminator check
    pub config: Account<'info, Config>,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub enabled: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Config is disabled")]
    ConfigDisabled,
}
