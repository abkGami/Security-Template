// ✅ SECURE IMPLEMENTATION - Use this pattern in production

use anchor_lang::prelude::*;

/// Secure withdraw function with proper signer verification
///
/// SOLUTION: This instruction properly verifies that the authority account
/// actually signed the transaction using Anchor's #[account(signer)] constraint.
///
/// Security improvements:
/// 1. Authority MUST sign the transaction (enforced by Anchor)
/// 2. Authority pubkey must match vault's stored authority
/// 3. Both checks must pass for withdrawal to succeed
///
/// An attacker cannot exploit this because:
/// - Even if they know the authority's pubkey
/// - Even if they include it in the transaction
/// - The transaction will fail unless signed by the authority's private key
pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ✅ SECURE: At this point, we know authority.is_signer == true
    // because of the #[account(signer)] constraint in WithdrawSecure struct
    
    // Double-check authority matches (defense in depth)
    // This check is redundant due to the constraint, but good for explicitness
    require!(
        vault.authority == ctx.accounts.authority.key(),
        ErrorCode::Unauthorized
    );
    
    // Check sufficient balance
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    require!(
        vault_lamports >= amount,
        ErrorCode::InsufficientFunds
    );
    
    // ✅ SAFE: Transfer executes only after verifying:
    // 1. Authority signed the transaction (Anchor constraint)
    // 2. Authority pubkey matches vault owner (explicit check)
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;
    
    // Update vault state
    vault.total_withdrawn = vault.total_withdrawn.checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    
    msg!("Securely withdrawn {} lamports from vault", amount);
    
    Ok(())
}

/// Alternative secure implementation using manual signer check
///
/// USE CASE: When you need more control or can't use Anchor constraints
pub fn withdraw_manual_check(ctx: Context<WithdrawManual>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ✅ CRITICAL: Manual signer verification
    // Check that the authority account has is_signer = true
    // This means the transaction was signed with this account's private key
    require!(
        ctx.accounts.authority.is_signer,
        ErrorCode::MissingSigner
    );
    
    // Check authority matches
    require!(
        vault.authority == ctx.accounts.authority.key(),
        ErrorCode::Unauthorized
    );
    
    // Check sufficient balance
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    require!(
        vault_lamports >= amount,
        ErrorCode::InsufficientFunds
    );
    
    // Safe transfer
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;
    
    vault.total_withdrawn = vault.total_withdrawn.checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    
    Ok(())
}

/// Secure account validation using Signer type
///
/// BEST PRACTICE: Use Signer<'info> type which enforces signature at compile time
#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    /// ✅ SOLUTION 1: Using Signer<'info> type
    /// The Signer type automatically enforces that this account signed the transaction
    /// This is checked by Anchor before your instruction code even runs
    /// 
    /// Compiler enforces: authority must be a signer
    /// Runtime enforces: authority.is_signer must be true
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Alternative secure validation using AccountInfo with signer constraint
///
/// BEST PRACTICE: Use #[account(signer)] constraint when using AccountInfo
#[derive(Accounts)]
pub struct WithdrawManual<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    /// ✅ SOLUTION 2: Using #[account(signer)] constraint
    /// This tells Anchor to verify is_signer == true before calling your function
    /// 
    /// The constraint checks:
    /// - Account must be present in transaction
    /// - Account must have signed the transaction
    /// - Fails immediately if is_signer == false
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}

/// Secure vault account structure with additional safety features
#[account]
pub struct Vault {
    /// The authority that can withdraw from this vault
    pub authority: Pubkey,
    
    /// Track total withdrawn for auditing and limits
    pub total_withdrawn: u64,
    
    /// Optional: Add a withdrawal limit for extra security
    pub withdrawal_limit: u64,
    
    /// Optional: Add a bump for PDA derivation if needed
    pub bump: u8,
}

impl Vault {
    /// Space calculation: 8 + 32 + 8 + 8 + 1 = 57 bytes
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1;
    
    /// Initialize a new vault with security defaults
    pub fn new(authority: Pubkey, withdrawal_limit: u64, bump: u8) -> Self {
        Self {
            authority,
            total_withdrawn: 0,
            withdrawal_limit,
            bump,
        }
    }
    
    /// Check if withdrawal would exceed limit
    pub fn can_withdraw(&self, amount: u64) -> bool {
        if self.withdrawal_limit == 0 {
            return true; // No limit set
        }
        
        self.total_withdrawn.saturating_add(amount) <= self.withdrawal_limit
    }
}

/// Enhanced error codes for secure implementation
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Authority mismatch")]
    Unauthorized,
    
    #[msg("Missing required signature")]
    MissingSigner,
    
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    
    #[msg("Withdrawal would exceed limit")]
    WithdrawalLimitExceeded,
    
    #[msg("Math overflow in calculation")]
    MathOverflow,
}

// ============================================================================
// SECURITY BEST PRACTICES DEMONSTRATED
// ============================================================================

/// Example: Initializing a vault securely
///
/// This shows proper initialization with authority verification
pub fn initialize_vault_secure(
    ctx: Context<InitializeVault>,
    withdrawal_limit: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // Set the authority to the transaction signer
    // Because initializer is Signer<'info>, we know they signed
    vault.authority = ctx.accounts.initializer.key();
    vault.total_withdrawn = 0;
    vault.withdrawal_limit = withdrawal_limit;
    vault.bump = ctx.bumps.vault; // Store PDA bump if using PDA
    
    msg!("Vault initialized with authority: {}", vault.authority);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// ✅ Signer type ensures initializer signed the transaction
    #[account(mut)]
    pub initializer: Signer<'info>,
    
    /// Initialize vault as a PDA for extra security
    #[account(
        init,
        payer = initializer,
        space = Vault::LEN,
        seeds = [b"vault", initializer.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    pub system_program: Program<'info, System>,
}

/// Example: Updating vault authority securely
///
/// Demonstrates proper authority transfer with signature checks
pub fn update_authority(
    ctx: Context<UpdateAuthority>,
    new_authority: Pubkey,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // ✅ At this point we know:
    // 1. current_authority is a Signer (signed the transaction)
    // 2. current_authority.key() matches vault.authority (from constraint)
    
    let old_authority = vault.authority;
    vault.authority = new_authority;
    
    msg!("Authority updated from {} to {}", old_authority, new_authority);
    
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub vault: Account<'info, Vault>,
    
    /// ✅ Both a Signer and checked against vault.authority
    /// This ensures:
    /// 1. The current authority signed this transaction
    /// 2. The signing account matches the vault's stored authority
    pub authority: Signer<'info>,
}
