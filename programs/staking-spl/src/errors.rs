use anchor_lang::error_code;

#[error_code]
pub enum DappError {
    #[msg("Insufficient stake to withdraw the requested amount.")]
    InsufficientStake,
}
