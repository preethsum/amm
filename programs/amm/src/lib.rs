#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Auh1xSS7jBH2jRVLRdcV4P7fiLh9nrXFcp1K9RVvCJQA");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, swap_fee: u16) -> Result<()> {
        ctx.accounts.process_initialize_pool(swap_fee, &ctx.bumps)
    }

    // User specify amount to lp tokens he/she wants to get form the give amount_x, amount_y liquidity
    pub fn add_liquidity(
        ctx: Context<Liquidity>,
        amount_x: u64,
        amount_y: u64,
        amount_lp: u64,
    ) -> Result<()> {
        ctx.accounts
            .process_add_liquidity(amount_x, amount_y, amount_lp)
    }

    pub fn remove_liquidity(
        ctx: Context<Liquidity>,
        amount_x: u64, // expected amount of x tokens on burning amount_lp lp tokens
        amount_y: u64, // expected amount of y tokens on burning amount_lp lp tokens
        amount_lp: u64,
    ) -> Result<()> {
        ctx.accounts
            .process_remove_liquidity(amount_x, amount_y, amount_lp)
    }

    // is_x specify the whether the swaping for is x or not
    // if swapping for x then amount_y becomes the input amount and the amount_x is min amount of x to get back
    pub fn swap(ctx: Context<Swap>, amount_x: u64, amount_y: u64, is_x: bool) -> Result<()> {
        ctx.accounts.process_swap(amount_x, amount_y, is_x)?;
        Ok(())
    }
}
