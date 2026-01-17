// ❌ VULNERABLE IMPLEMENTATION
use anchor_lang::prelude::*;

/// VULNERABILITY: Unchecked arithmetic operations
/// 
/// In Rust release builds, integer overflow wraps around by default.
/// This can lead to:
/// - Balance manipulation
/// - Unlimited token minting
/// - Reward pool drainage
/// - Broken accounting
pub fn deposit_insecure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ⚠️ DANGER: Unchecked addition - can overflow and wrap
    // If total_deposited = u64::MAX and amount = 1
    // Result wraps to 0 instead of erroring
    vault.total_deposited = vault.total_deposited + amount;
    
    msg!("Deposited {} tokens (INSECURE - can overflow!)", amount);
    Ok(())
}

pub fn withdraw_insecure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ⚠️ DANGER: Unchecked subtraction - can underflow and wrap
    // If total_deposited = 100 and amount = 101
    // Result wraps to u64::MAX instead of erroring
    vault.total_deposited = vault.total_deposited - amount;
    
    msg!("Withdrew {} tokens (INSECURE - can underflow!)", amount);
    Ok(())
}

pub fn calculate_rewards_insecure(ctx: Context<VaultOps>, multiplier: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ⚠️ DANGER: Unchecked multiplication - can overflow
    let rewards = vault.total_deposited * multiplier;
    
    // ⚠️ DANGER: Unchecked addition in reward distribution
    vault.total_rewards = vault.total_rewards + rewards;
    
    msg!("Calculated {} rewards (INSECURE - can overflow!)", rewards);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultOps<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    pub authority: Signer<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub total_rewards: u64,
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8;
}
