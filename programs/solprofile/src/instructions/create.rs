use anchor_lang::prelude::*;

use crate::states::{Profile, PROFILE_SEED};

pub fn create(_: Context<Create>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = owner, space = Profile::MAX_SIZE, seeds=[PROFILE_SEED, owner.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn get_profile_account(owner: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PROFILE_SEED, owner.to_bytes().as_ref()], program_id)
}
