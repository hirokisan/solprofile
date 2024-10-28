use anchor_lang::prelude::*;

use crate::states::{Profile, PROFILE_SEED};

pub fn update(ctx: Context<Update>, args: UpdateArgs) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    if args.name.is_some() {
        profile.name = args.name.unwrap();
    }
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateArgs {
    pub name: Option<String>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, seeds=[PROFILE_SEED, owner.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
