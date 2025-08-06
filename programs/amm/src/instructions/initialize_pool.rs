use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::AmmError, Pool, ANCHOR_DISCRIMINATOR, LP_SEED, POOL_SEED};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mint::token_program= token_program
    )]
    pub mint_x: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::token_program = associated_token_program,
        // Since this program is not creating the ata we are givin the auth to config, if this program creates then we can assign ata as authority
        associated_token::authority = pool,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::token_program = associated_token_program,
        associated_token::authority = pool,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        space = ANCHOR_DISCRIMINATOR + Pool::INIT_SPACE,
        seeds = [POOL_SEED, mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = initializer,
        mint::token_program = token_program,
        // Here if need mint_lp can also be the authority
        mint::authority = pool,
        mint::decimals = 9,
        seeds = [LP_SEED, pool.key().as_ref()],
        bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePool<'info> {
    pub fn process_initialize_pool(
        &mut self,
        swap_fee: u16,
        bumps: &InitializePoolBumps,
    ) -> Result<()> {
        require!(
            self.mint_x.key() != self.mint_y.key(),
            AmmError::SimilarMints
        );
        require!(swap_fee <= 1, AmmError::InvalidSwapFee);
        self.pool.set_inner(Pool {
            initializer: self.initializer.key(),
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            is_locked: false,
            swap_fee,
            lp_bump: bumps.mint_lp,
            bump: bumps.pool,
        });
        Ok(())
    }
}
