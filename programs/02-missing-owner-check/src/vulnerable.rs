// ❌ VULNERABLE IMPLEMENTATION
use anchor_lang::prelude::*;

/// VULNERABILITY: No owner verification on accounts
/// 
/// This allows attackers to pass fake accounts owned by their own programs
/// The program treats these fake accounts as legitimate, leading to:
/// - Bypass of balance checks
/// - Fake state data
/// - Unauthorized operations
pub fn process_payment_insecure(ctx: Context<PaymentInsecure>, amount: u64) -> Result<()> {
    // ⚠️ DANGER: user_token_account is AccountInfo with no owner check
    // Attacker can pass an account owned by their malicious program
    // that returns fake balance data
    
    let token_data = ctx.accounts.user_token_account.try_borrow_data()?;
    
    // Manually parsing token account data (dangerous!)
    // If attacker controls the account, they control this data
    let balance = u64::from_le_bytes(
        token_data[64..72].try_into()
            .map_err(|_| ErrorCode::InvalidTokenAccount)?
    );
    
    // ⚠️ This check can be bypassed with fake account
    require!(balance >= amount, ErrorCode::InsufficientBalance);
    
    msg!("Processing payment of {} tokens (INSECURE)", amount);
    
    Ok(())
}

#[derive(Accounts)]
pub struct PaymentInsecure<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// ⚠️ VULNERABILITY: Using AccountInfo without owner check
    /// Should verify owner == spl_token::ID
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub vault: Account<'info, PaymentVault>,
}

#[account]
pub struct PaymentVault {
    pub authority: Pubkey,
    pub total_collected: u64,
}

impl PaymentVault {
    pub const LEN: usize = 8 + 32 + 8;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
}
