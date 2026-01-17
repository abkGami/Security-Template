use anchor_lang::prelude::*;
pub mod vulnerable;
pub mod secure;
pub use vulnerable::*;
pub use secure::*;

declare_id!("CnV2bXqZ5F7kL8TmW3R9YpE4HxG6JaNf2DsU7BwK5Mqh");

#[program]
pub mod type_cosplay {
    use super::*;
    
    pub fn process_vulnerable(ctx: Context<ProcessVulnerable>) -> Result<()> {
        vulnerable::process_vulnerable(ctx)
    }
    
    pub fn process_secure(ctx: Context<ProcessSecure>) -> Result<()> {
        secure::process_secure(ctx)
    }
}
