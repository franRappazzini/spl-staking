use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constants::GLOBAL_STATE_SEED, errors::DappError, events, states::GlobalState};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump = global_state.bump,
        has_one = authority,
        constraint = global_state.treasury_amount > 0 @ DappError::NoRewardsAvailable,
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
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program,
    )]
    pub authority_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(address = global_state.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFees<'info> {
    pub fn withdraw_fees(&mut self) -> Result<()> {
        let amount = self.global_state.treasury_amount;
        self.global_state.treasury_amount = 0;
        self.global_state.last_update_time = Clock::get()?.unix_timestamp;

        // withdraw fees
        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL_STATE_SEED, &[self.global_state.bump]]];

        let cpi_accounts = token::Transfer {
            authority: self.global_state.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.authority_token_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;

        emit!(events::WithdrawFeesEvent {
            authority: self.authority.key(),
            authority_token_account: self.authority_token_account.key(),
            amount,
        });

        Ok(())
    }
}
