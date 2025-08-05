use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Mints should not be same")]
    SimilarMints,
    #[msg("Swap fee should be less than or equal to one")]
    InvalidSwapFee,
    #[msg("Amount cannot be zero")]
    InvalidAmount,
}
