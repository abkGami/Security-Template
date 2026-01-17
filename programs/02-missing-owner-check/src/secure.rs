// ✅ SECURE IMPLEMENTATION
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

/// SOLUTION: Using Anchor's Account<'info, TokenAccount> type
/// 
/// This automatically verifies:
/// 1. Account owner is spl_token::ID
/// 2. Account data deserializes correctly as TokenAccount
/// 3. Account discriminator is valid
pub fn process_payment_secure(ctx: Context<PaymentSecure>, amount: u64) -> Result<()> {
    // ✅ SAFE: token_account is validated by Anchor
    // We know it's owned by Token Program and data is valid
    let token_account = &ctx.accounts.user_token_account;
    
    require!(
        token_account.amount >= amount,
        ErrorCode::InsufficientBalance
    );
    
    // Additional validation: verify token account belongs to user
    require!(
        token_account.owner == ctx.accounts.user.key(),
        ErrorCode::InvalidTokenOwner
    );
    
    msg!("Processing payment of {} tokens (SECURE)", amount);
    
    Ok(())
}

/// Manual owner check approach (alternative)
pub fn process_payment_manual(ctx: Context<PaymentManual>, amount: u64) -> Result<()> {
    // ✅ SOLUTION: Manual owner verification before using account
    require!(
        ctx.accounts.user_token_account.owner == &spl_token::ID,
        ErrorCode::InvalidOwner
    );
    
    // Now safe to deserialize
    let token_account = TokenAccount::try_deserialize(
        &mut &ctx.accounts.user_token_account.data.borrow()[..]
    )?;
    
    require!(
        token_account.amount >= amount,
        ErrorCode::InsufficientBalance
    );
    
    Ok(())
}

#[derive(Accounts)]
pub struct PaymentSecure<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// ✅ SOLUTION: Account<'info, TokenAccount> enforces owner check
    /// Anchor verifies owner == Token Program automatically
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ ErrorCode::InvalidTokenOwner
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vault: Account<'info, PaymentVault>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PaymentManual<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// Using AccountInfo with manual check in instruction
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
    #[msg("Invalid owner for account")]
    InvalidOwner,
    #[msg("Token account owner mismatch")]
    InvalidTokenOwner,
}
