use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{constants::GLOBAL_STATE_SEED, states::GlobalState};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = GlobalState::SIZE,
        seeds = [GLOBAL_STATE_SEED],
        bump
        
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority, // to mint from ts tests
        mint::freeze_authority = authority, // to mint from ts tests
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = global_state,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, reward_rate: u64, bump: u8) -> Result<()> {
        self.global_state.set_inner(GlobalState {
            authority: self.authority.key(),
            mint: self.mint.key(),
            vault: self.vault.key(),
            total_staked: 0,
            acc_reward_per_share: 0,
            last_update_time: 0,
            reward_rate,
            bump,
        });

        Ok(())
    }
}
