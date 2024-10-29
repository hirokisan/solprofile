use anchor_lang::prelude::*;

pub fn get_default_signer_keypair_path() -> String {
    shellexpand::tilde("~/.config/solana/id.json").to_string()
}

pub fn get_profile_account(owner: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    pub const PROFILE_SEED: &'static [u8] = b"profile";
    Pubkey::find_program_address(&[PROFILE_SEED, owner.to_bytes().as_ref()], program_id)
}
