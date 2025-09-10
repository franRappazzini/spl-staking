use anchor_lang::prelude::*;

#[event]
pub struct DepositSPLEvent {
    pub user: Pubkey,
    pub user_token_account: Pubkey,
    pub stake_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ClaimRewardsSPLEvent {
    pub user: Pubkey,
    pub user_token_account: Pubkey,
    pub stake_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ClosePositionSPLEvent {
    pub user: Pubkey,
    pub user_token_account: Pubkey,
    pub closed_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawFeesEvent {
    pub authority: Pubkey,
    pub authority_token_account: Pubkey,
    pub amount: u64,
}
