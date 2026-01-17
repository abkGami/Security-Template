use anchor_lang::prelude::*;

pub mod vulnerable;
pub mod secure;

pub use vulnerable::*;
pub use secure::*;

declare_id!("GMDYqj3bRYPjUCKPF6wHPUxZZgWDhPPKxdTfUmFN8jj8");

#[program]
pub mod missing_owner_check {
    use super::*;

    /// ❌ VULNERABLE: Process payment without owner check
    pub fn process_payment_insecure(ctx: Context<PaymentInsecure>, amount: u64) -> Result<()> {
        vulnerable::process_payment_insecure(ctx, amount)
    }
    
    /// ✅ SECURE: Process payment with Anchor type validation
    pub fn process_payment_secure(ctx: Context<PaymentSecure>, amount: u64) -> Result<()> {
        secure::process_payment_secure(ctx, amount)
    }
    
    /// ✅ SECURE: Process payment with manual owner check
    pub fn process_payment_manual(ctx: Context<PaymentManual>, amount: u64) -> Result<()> {
        secure::process_payment_manual(ctx, amount)
    }
}
