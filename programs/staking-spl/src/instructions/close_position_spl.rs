use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{GLOBAL_STATE_SEED, PRECISION, STAKE_SEED},
    errors::DappError,
    events,
    states::{GlobalState, Stake},
};

#[derive(Accounts)]
pub struct ClosePositionSpl<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub closer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [STAKE_SEED, closer.key().as_ref()],
        bump = stake.bump,
        close = closer
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
        associated_token::authority = closer,
        associated_token::token_program = token_program,
    )]
    pub closer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, address = global_state.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClosePositionSpl<'info> {
    pub fn close_position_spl(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let time_diff = (now - self.global_state.last_update_time) as u64;

        // update global state
        if self.global_state.total_staked > 0 {
            let rewards = time_diff * self.global_state.reward_rate;
            self.global_state.acc_reward_per_share +=
                (rewards as u128 * PRECISION) / self.global_state.total_staked as u128;
        }
        self.global_state.last_update_time = now;

        // update global state
        self.global_state.total_staked = self
            .global_state
            .total_staked
            .saturating_sub(self.stake.amount);

        // transfer staked tokens back to user
        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL_STATE_SEED, &[self.global_state.bump]]];

        let cpi_accounts = token::Transfer {
            authority: self.global_state.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.closer_token_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, self.stake.amount)?;

        // calculate pending rewards
        let pending_rewards = ((self.stake.amount as u128 * self.global_state.acc_reward_per_share)
            / PRECISION) as u64
            - self.stake.reward_debt;

        let total_rewards = self.stake.unclaimed_rewards + pending_rewards;

        // optionally mint rewards
        if total_rewards > 0 {
            self.stake.unclaimed_rewards = 0;

            let cpi_accounts = token::MintTo {
                authority: self.authority.to_account_info(),
                to: self.closer_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

            token::mint_to(cpi_ctx, total_rewards)?;
        }

        emit!(events::ClosePositionSPLEvent {
            user: self.closer.key(),
            user_token_account: self.closer_token_account.key(),
            closed_account: self.stake.key(),
            amount: total_rewards + self.stake.amount,
        });

        Ok(())
    }
}
