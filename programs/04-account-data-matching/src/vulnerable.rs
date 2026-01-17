// ❌ VULNERABLE - Missing PDA verification
use anchor_lang::prelude::*;

pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
    // ⚠️ VULNERABILITY: No verification that user_stats is correctly derived PDA
    // Attacker can create fake user_stats with inflated balance
    let user_stats = &ctx.accounts.user_stats;
    
    require!(user_stats.balance >= amount, ErrorCode::InsufficientBalance);
    
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawInsecure<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// ⚠️ NO seeds/bump verification - accepts any account!
    #[account(mut)]
    pub user_stats: Account<'info, UserStats>,
    
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}

#[account]
pub struct UserStats {
    pub user: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl UserStats {
    pub const LEN: usize = 8 + 32 + 8 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}
