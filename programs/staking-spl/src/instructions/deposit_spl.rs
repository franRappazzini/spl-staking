use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{GLOBAL_STATE_SEED, PRECISION, STAKE_SEED},
    states::{GlobalState, Stake},
};

#[derive(Accounts)]
pub struct DepositSPL<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump = global_state.bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = global_state,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = Stake::SIZE,
        seeds = [STAKE_SEED, depositor.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, Stake>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(address = global_state.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositSPL<'info> {
    pub fn deposit_spl(&mut self, amount: u64, bump: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let time_diff = (now - self.global_state.last_update_time) as u64;

        // update global state
        if self.global_state.total_staked > 0 {
            let rewards = time_diff * self.global_state.reward_rate;
            self.global_state.acc_reward_per_share +=
                (rewards as u128 * PRECISION) / self.global_state.total_staked as u128;
        }
        self.global_state.last_update_time = now;
        self.global_state.total_staked += amount;

        // update user stake
        if self.stake.amount > 0 {
            let pending_rewards = ((self.stake.amount as u128
                * self.global_state.acc_reward_per_share)
                / PRECISION) as u64
                - self.stake.reward_debt;
            if pending_rewards > 0 {
                self.stake.unclaimed_rewards += pending_rewards;
            }

            self.stake.amount += amount;
            self.stake.reward_debt = ((self.stake.amount as u128
                * self.global_state.acc_reward_per_share)
                / PRECISION) as u64;
            self.stake.last_update_time = now;
        } else {
            self.stake.set_inner(Stake {
                owner: self.depositor.key(),
                amount,
                start_time: now,
                last_update_time: now,
                reward_debt: ((amount as u128 * self.global_state.acc_reward_per_share) / PRECISION)
                    as u64,
                unclaimed_rewards: 0,
                bump,
            });
        }

        // transfer tokens from user to vault
        let cpi_accounts = token::Transfer {
            authority: self.depositor.to_account_info(),
            from: self.depositor_token_account.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token::transfer(cpi_ctx, amount)
    }
}
