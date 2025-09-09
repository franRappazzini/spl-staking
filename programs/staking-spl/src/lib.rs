mod constants;
mod errors;
mod instructions;
mod states;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5Eizeiy2uZEagP5f6FN7yq9rvGfdD9Z7qh8WrTg86eEL");

#[program]
pub mod staking_spl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, reward_rate: u64) -> Result<()> {
        ctx.accounts.initialize(reward_rate, ctx.bumps.global_state)
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_spl(amount, ctx.bumps.stake)
    }

    pub fn claim_rewards_spl(ctx: Context<ClaimRewardsSpl>) -> Result<()> {
        ctx.accounts.claim_rewards_spl()
    }

    pub fn close_position_spl(ctx: Context<ClosePositionSpl>) -> Result<()> {
        ctx.accounts.close_position_spl()
    }
}
