// ✅ SECURE IMPLEMENTATION
use anchor_lang::prelude::*;

/// SOLUTION: Using checked arithmetic operations
/// 
/// checked_* methods return Option<T>:
/// - Some(result) if operation succeeds
/// - None if overflow/underflow would occur
/// 
/// This prevents silent wrapping and ensures errors are caught
pub fn deposit_secure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ✅ SAFE: checked_add returns None on overflow
    vault.total_deposited = vault.total_deposited
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    
    msg!("Securely deposited {} tokens", amount);
    Ok(())
}

pub fn withdraw_secure(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ✅ SAFE: checked_sub returns None on underflow
    vault.total_deposited = vault.total_deposited
        .checked_sub(amount)
        .ok_or(ErrorCode::MathUnderflow)?;
    
    // Also update withdrawal tracking
    vault.total_withdrawn = vault.total_withdrawn
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    
    msg!("Securely withdrew {} tokens", amount);
    Ok(())
}

pub fn calculate_rewards_secure(ctx: Context<VaultOps>, multiplier: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ✅ SAFE: Chain multiple checked operations
    let rewards = vault.total_deposited
        .checked_mul(multiplier)
        .ok_or(ErrorCode::MathOverflow)?;
    
    vault.total_rewards = vault.total_rewards
        .checked_add(rewards)
        .ok_or(ErrorCode::MathOverflow)?;
    
    msg!("Securely calculated {} rewards", rewards);
    Ok(())
}

/// Example: Complex calculation with multiple safety checks
pub fn compound_interest_secure(
    ctx: Context<VaultOps>,
    rate_numerator: u64,
    rate_denominator: u64,
    periods: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // Validate inputs
    require!(rate_denominator > 0, ErrorCode::DivisionByZero);
    require!(periods > 0, ErrorCode::InvalidPeriods);
    
    let principal = vault.total_deposited;
    let mut amount = principal;
    
    // Calculate compound interest safely
    for _ in 0..periods {
        // amount = amount * (1 + rate)
        let interest = amount
            .checked_mul(rate_numerator)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(rate_denominator)
            .ok_or(ErrorCode::MathOverflow)?;
        
        amount = amount
            .checked_add(interest)
            .ok_or(ErrorCode::MathOverflow)?;
    }
    
    // Calculate and add total rewards
    let total_rewards = amount
        .checked_sub(principal)
        .ok_or(ErrorCode::MathUnderflow)?;
    
    vault.total_rewards = vault.total_rewards
        .checked_add(total_rewards)
        .ok_or(ErrorCode::MathOverflow)?;
    
    msg!("Compound interest calculated: {} rewards", total_rewards);
    Ok(())
}

/// Alternative: Using saturating arithmetic (caps at max/min)
/// Use when you want to cap rather than error
pub fn deposit_saturating(ctx: Context<VaultOps>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // Saturating: maxes out at u64::MAX instead of wrapping or erroring
    vault.total_deposited = vault.total_deposited.saturating_add(amount);
    
    msg!("Deposited with saturation: {}", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultOps<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::LEN
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub total_rewards: u64,
    pub max_deposit: u64,  // Optional: enforce deposit limits
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8;
    
    pub fn new(authority: Pubkey) -> Self {
        Self {
            authority,
            total_deposited: 0,
            total_withdrawn: 0,
            total_rewards: 0,
            max_deposit: u64::MAX,
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math operation resulted in overflow")]
    MathOverflow,
    
    #[msg("Math operation resulted in underflow")]
    MathUnderflow,
    
    #[msg("Division by zero attempted")]
    DivisionByZero,
    
    #[msg("Invalid number of periods")]
    InvalidPeriods,
}
