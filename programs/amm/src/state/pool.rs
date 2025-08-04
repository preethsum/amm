use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub initializer: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub is_locked: bool,
    pub swap_fee: u16,
    pub lp_bump: u8,
    pub bump: u8,
}
