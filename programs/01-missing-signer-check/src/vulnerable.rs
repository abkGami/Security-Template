// ❌ VULNERABLE IMPLEMENTATION - DO NOT USE IN PRODUCTION
// This module demonstrates a CRITICAL missing signer check vulnerability

use anchor_lang::prelude::*;

/// Vulnerable withdraw function that lacks proper signer verification
///
/// VULNERABILITY: This instruction only checks if the authority pubkey matches
/// the vault's stored authority, but it NEVER verifies that the authority
/// actually signed this transaction.
///
/// An attacker can:
/// 1. Read the vault's authority pubkey from on-chain data
/// 2. Create a transaction that includes that pubkey as an account
/// 3. Sign with their own keypair (not the authority's)
/// 4. Successfully drain the vault because there's no signature check
///
/// Real-world impact: Wormhole Bridge lost $325M due to similar missing
/// signature verification in February 2022.
pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ⚠️ CRITICAL FLAW: This only checks if the pubkeys match
    // It does NOT verify that the authority account signed this transaction
    if vault.authority != ctx.accounts.authority.key() {
        return Err(ErrorCode::Unauthorized.into());
    }
    
    // Check if vault has sufficient balance
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    if vault_lamports < amount {
        return Err(ErrorCode::InsufficientFunds.into());
    }
    
    // ⚠️ DANGER: Transfer executes without verifying authority's signature
    // An attacker who knows the authority's pubkey can drain this vault
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;
    
    msg!("Withdrawn {} lamports from vault (INSECURE)", amount);
    
    Ok(())
}

/// Vulnerable account validation structure
///
/// PROBLEM: Using AccountInfo<'info> for authority with no signer constraint
#[derive(Accounts)]
pub struct WithdrawInsecure<'info> {
    /// The vault account holding funds
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    /// ⚠️ VULNERABILITY: No #[account(signer)] constraint!
    /// Should be: #[account(signer)] or use Signer<'info> type
    /// 
    /// Without signer verification, anyone can pass any pubkey here
    /// and the program will accept it as long as it matches vault.authority
    #[account(mut)]
    pub authority: AccountInfo<'info>,
}

/// Vault account structure
#[account]
pub struct Vault {
    /// The authority that can withdraw from this vault
    pub authority: Pubkey,
    /// Track total withdrawn for auditing
    pub total_withdrawn: u64,
}

impl Vault {
    /// Space calculation for account allocation
    /// 8 (discriminator) + 32 (authority) + 8 (total_withdrawn)
    pub const LEN: usize = 8 + 32 + 8;
}

/// Custom error codes for the vulnerable program
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Authority mismatch")]
    Unauthorized,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
}
