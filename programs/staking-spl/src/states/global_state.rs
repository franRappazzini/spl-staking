use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR;

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub total_staked: u64,
    pub acc_reward_per_share: u128,
    pub last_update_time: i64,
    pub reward_rate: u64,
    pub bump: u8,
}

impl GlobalState {
    pub const SIZE: usize = DISCRIMINATOR + GlobalState::INIT_SPACE;
}
