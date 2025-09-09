use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{GLOBAL_STATE_SEED, PRECISION, STAKE_SEED},
    states::{GlobalState, Stake},
};

#[derive(Accounts)]
pub struct ClaimRewardsSpl<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [STAKE_SEED, claimer.key().as_ref()],
        bump = stake.bump,
    )]
    pub stake: Account<'info, Stake>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = global_state,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = claimer,
        associated_token::token_program = token_program,
    )]
    pub claimer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, address = global_state.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimRewardsSpl<'info> {
    pub fn claim_rewards_spl(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let time_diff = (now - self.global_state.last_update_time) as u64;

        // update global state
        if self.global_state.total_staked > 0 {
            let rewards = time_diff * self.global_state.reward_rate;
            self.global_state.acc_reward_per_share +=
                (rewards as u128 * PRECISION) / self.global_state.total_staked as u128;
        }
        self.global_state.last_update_time = now;

        let pending_rewards = ((self.stake.amount as u128 * self.global_state.acc_reward_per_share)
            / PRECISION) as u64
            - self.stake.reward_debt;

        if pending_rewards > 0 {
            self.stake.unclaimed_rewards += pending_rewards;
        }

        let total_rewards = self.stake.unclaimed_rewards;

        if total_rewards > 0 {
            msg!(
                "Transferring {} rewards to {}",
                total_rewards,
                self.claimer.key()
            );

            let cpi_accounts = token::MintTo {
                authority: self.authority.to_account_info(),
                to: self.claimer_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

            token::mint_to(cpi_ctx, total_rewards)?;

            // update state
            self.stake.unclaimed_rewards = 0;
        }

        self.stake.reward_debt = ((self.stake.amount as u128
            * self.global_state.acc_reward_per_share)
            / PRECISION) as u64;
        self.stake.last_update_time = now;

        Ok(())
    }
}
