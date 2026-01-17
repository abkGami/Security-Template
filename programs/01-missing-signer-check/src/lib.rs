use anchor_lang::prelude::*;

// Import vulnerable and secure implementations
pub mod vulnerable;
pub mod secure;

// Re-export for easy access
pub use vulnerable::*;
pub use secure::*;

declare_id!("HVDgr5PCwyH1bGkVvB6sDqzDkjZq6FfC8qE38PV2Z8Fc");

#[program]
pub mod missing_signer_check {
    use super::*;

    // ========================================================================
    // VULNERABLE INSTRUCTIONS - For educational testing only
    // ========================================================================
    
    /// ❌ VULNERABLE: Withdraw without signer verification
    pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
        vulnerable::withdraw_insecure(ctx, amount)
    }
    
    // ========================================================================
    // SECURE INSTRUCTIONS - Use these patterns in production
    // ========================================================================
    
    /// ✅ SECURE: Withdraw with Signer type constraint
    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        secure::withdraw_secure(ctx, amount)
    }
    
    /// ✅ SECURE: Withdraw with manual signer check
    pub fn withdraw_manual_check(ctx: Context<WithdrawManual>, amount: u64) -> Result<()> {
        secure::withdraw_manual_check(ctx, amount)
    }
    
    /// ✅ SECURE: Initialize vault with proper authority
    pub fn initialize_vault_secure(
        ctx: Context<InitializeVault>,
        withdrawal_limit: u64,
    ) -> Result<()> {
        secure::initialize_vault_secure(ctx, withdrawal_limit)
    }
    
    /// ✅ SECURE: Update vault authority with verification
    pub fn update_authority(
        ctx: Context<UpdateAuthority>,
        new_authority: Pubkey,
    ) -> Result<()> {
        secure::update_authority(ctx, new_authority)
    }
}
