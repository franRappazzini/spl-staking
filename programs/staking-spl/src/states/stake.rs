use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR;

#[account]
#[derive(InitSpace)]
pub struct Stake {
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub last_update_time: i64,
    pub reward_debt: u64,
    pub unclaimed_rewards: u64,
    pub bump: u8,
}

impl Stake {
    pub const SIZE: usize = DISCRIMINATOR + Stake::INIT_SPACE;
}
