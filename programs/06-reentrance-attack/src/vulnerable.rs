// ❌ VULNERABLE - External call before state update
use anchor_lang::prelude::*;

pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>, amount: u64) -> Result<()> {
    let user = &ctx.accounts.user;
    require!(user.balance >= amount, ErrorCode::InsufficientBalance);
    
    // ⚠️ DANGER: External call BEFORE state update
    // Attacker can recursively call withdraw
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **user.to_account_info().try_borrow_mut_lamports()? += amount;
    
    // ⚠️ State updated AFTER external call - too late!
    let user_account = &mut ctx.accounts.user_account;
    user_account.balance = user_account.balance.checked_sub(amount).unwrap();
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}

#[account]
pub struct UserAccount {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}
