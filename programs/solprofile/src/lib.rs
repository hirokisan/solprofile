use anchor_lang::prelude::*;

declare_id!("EfwU21VVpG9aHLmYAM2zcEaPEDSRfXxCGv3S95zTNp8h");

#[program]
pub mod solprofile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
