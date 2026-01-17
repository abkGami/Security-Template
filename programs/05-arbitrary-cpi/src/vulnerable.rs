// ❌ VULNERABLE - Accepts arbitrary program IDs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

pub fn transfer_insecure(ctx: Context<TransferInsecure>, amount: u64) -> Result<()> {
    // ⚠️ DANGER: Accepts any program ID from user
    // Attacker can pass malicious program that doesn't transfer tokens
    
    let transfer_ix = Instruction {
        program_id: ctx.accounts.token_program.key(),  // User-controlled!
        accounts: vec![/* ... */],
        data: vec![/* ... */],
    };
    
    invoke(&transfer_ix, &[])?;
    msg!("Transfer completed (INSECURE)");
    Ok(())
}

#[derive(Accounts)]
pub struct TransferInsecure<'info> {
    /// ⚠️ No validation - any program accepted
    pub token_program: AccountInfo<'info>,
    pub from: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub authority: Signer<'info>,
}
