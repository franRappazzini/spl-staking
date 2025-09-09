use anchor_lang::constant;

pub const DISCRIMINATOR: usize = 8;

#[constant]
pub const GLOBAL_STATE_SEED: &[u8] = b"global_state";

#[constant]
pub const STAKE_SEED: &[u8] = b"stake";

#[constant]
pub const PRECISION: u128 = 1e12 as u128;
