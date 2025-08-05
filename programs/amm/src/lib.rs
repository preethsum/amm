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
}
