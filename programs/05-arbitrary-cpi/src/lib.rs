use anchor_lang::prelude::*;
pub mod vulnerable;
pub mod secure;
pub use vulnerable::*;
pub use secure::*;

declare_id!("D3fWpLnJg5F6xN8E7vQ2cYZhB5XmK4RxL9TaPbV2Jn5s");

#[program]
pub mod arbitrary_cpi {
    use super::*;
    
    pub fn transfer_insecure(ctx: Context<TransferInsecure>, amount: u64) -> Result<()> {
        vulnerable::transfer_insecure(ctx, amount)
    }
    
    pub fn transfer_secure(ctx: Context<TransferSecure>, amount: u64) -> Result<()> {
        secure::transfer_secure(ctx, amount)
    }
}
