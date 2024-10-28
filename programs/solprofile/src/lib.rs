use anchor_lang::prelude::*;

use crate::instructions::*;

pub mod instructions;
pub mod states;

declare_id!("EfwU21VVpG9aHLmYAM2zcEaPEDSRfXxCGv3S95zTNp8h");

#[program]
pub mod solprofile {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        instructions::create(ctx)
    }

    pub fn update(ctx: Context<Update>, args: UpdateArgs) -> Result<()> {
        instructions::update(ctx, args)
    }
}
