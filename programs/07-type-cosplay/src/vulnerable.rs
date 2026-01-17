// ❌ VULNERABLE - No discriminator check
use anchor_lang::prelude::*;

pub fn process_vulnerable(ctx: Context<ProcessVulnerable>) -> Result<()> {
    // ⚠️ Manual deserialization without type checking
    // Attacker can pass wrong account type with similar data layout
    let data = ctx.accounts.config_account.data.borrow();
    let parsed_config: Config = try_from_slice(&data[8..])?;
    
    msg!("Processing with admin: {}", parsed_config.admin);
    Ok(())
}

#[derive(Accounts)]
pub struct ProcessVulnerable<'info> {
    /// ⚠️ AccountInfo without type validation
    pub config_account: AccountInfo<'info>,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub enabled: bool,
}
