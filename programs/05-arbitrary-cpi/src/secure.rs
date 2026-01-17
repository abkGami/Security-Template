// ✅ SECURE - Validates program IDs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn transfer_secure(ctx: Context<TransferSecure>, amount: u64) -> Result<()> {
    // ✅ token_program is validated by Anchor
    // Only spl_token::ID is accepted
    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }
        ),
        amount
    )?;
    
    msg!("Transfer completed (SECURE)");
    Ok(())
}

#[derive(Accounts)]
pub struct TransferSecure<'info> {
    /// ✅ Program<'info, Token> validates program ID
    pub token_program: Program<'info, Token>,
    
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
}
