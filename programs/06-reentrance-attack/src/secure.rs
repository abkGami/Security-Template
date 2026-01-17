// ✅ SECURE - Checks-Effects-Interactions pattern
use anchor_lang::prelude::*;

pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    
    // ✅ CHECKS - Validate all conditions first
    require!(user_account.balance >= amount, ErrorCode::InsufficientBalance);
    require!(amount > 0, ErrorCode::InvalidAmount);
    
    // ✅ EFFECTS - Update state BEFORE external calls
    user_account.balance = user_account.balance
        .checked_sub(amount)
        .ok_or(ErrorCode::MathUnderflow)?;
    
    // ✅ INTERACTIONS - External calls LAST
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;
    
    msg!("Withdrawal completed securely");
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        has_one = user @ ErrorCode::Unauthorized
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}

#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Math underflow")]
    MathUnderflow,
    #[msg("Unauthorized")]
    Unauthorized,
}
