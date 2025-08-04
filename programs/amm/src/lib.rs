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
}
