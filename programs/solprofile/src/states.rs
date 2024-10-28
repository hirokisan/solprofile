use anchor_lang::prelude::*;

pub const PROFILE_SEED: &[u8] = b"profile";

// Space Reference : https://book.anchor-lang.com/anchor_references/space.html
pub const DISCRIMINATOR_SIZE: usize = 8;

#[account]
pub struct Profile {
    pub name: String,
}

impl Profile {
    pub const MAX_SIZE: usize = DISCRIMINATOR_SIZE + Self::MAX_NAME_SIZE;
    pub const MAX_NAME_SIZE: usize = 4 + 32;
}
