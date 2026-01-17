// ✅ SECURE - Proper PDA verification
use anchor_lang::prelude::*;

pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
    // ✅ user_stats PDA is verified by Anchor constraints
    let user_stats = &ctx.accounts.user_stats;
    
    require!(user_stats.balance >= amount, ErrorCode::InsufficientBalance);
    
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;
    
    Ok(())
}

pub fn initialize_user_stats(ctx: Context<InitializeUserStats>) -> Result<()> {
    let user_stats = &mut ctx.accounts.user_stats;
    user_stats.user = ctx.accounts.user.key();
    user_stats.balance = 0;
    user_stats.bump = ctx.bumps.user_stats;
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// ✅ Verifies PDA derivation with seeds and bump
    #[account(
        mut,
        seeds = [b"user_stats", user.key().as_ref()],
        bump = user_stats.bump,
        has_one = user @ ErrorCode::Unauthorized
    )]
    pub user_stats: Account<'info, UserStats>,
    
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = UserStats::LEN,
        seeds = [b"user_stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    
    pub system_program: Program<'info, System>,
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
    #[msg("Unauthorized")]
    Unauthorized,
}
