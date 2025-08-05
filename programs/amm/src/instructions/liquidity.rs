use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use constant_product_curve::{ConstantProduct, XYAmounts};

use crate::{error::AmmError, mint_lp, transfer_tokens, Pool, LP_SEED, POOL_SEED};

#[derive(Accounts)]
pub struct Liquidity<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program= token_program
    )]
    pub mint_x: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = mint_x,
        associated_token::token_program = associated_token_program,
        associated_token::authority = pool,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_y,
        associated_token::token_program = associated_token_program,
        associated_token::authority = pool,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program,
        mint::authority = pool,
        mint::decimals = 9,
        seeds = [LP_SEED, pool.key().as_ref()],
        bump = pool.lp_bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_lp: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [POOL_SEED, mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Liquidity<'info> {
    pub fn process_add_liquidity(
        &self,
        amount_x: u64,
        amount_y: u64,
        amount_lp: u64,
    ) -> Result<()> {
        // This function calculates the amount_x, amount_y required to be transfered to the respective vaults, and transfer them.
        require!(
            amount_x != 0 && amount_y != 0 && amount_lp != 0,
            AmmError::InvalidAmount
        );
        let (required_x, required_y): (u64, u64) = match self.mint_lp.supply == 0
            && self.vault_x.amount == 0
            && self.vault_y.amount == 0
        {
            true => (amount_x, amount_y),
            false => {
                let xy_amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.mint_x.supply,
                    self.mint_y.supply,
                    amount_lp,
                    self.mint_lp.supply,
                    9,
                )
                .unwrap();
                (xy_amounts.x, xy_amounts.y)
            }
        };

        if required_x > amount_x || required_y > amount_y {
            return Err(AmmError::InvalidAmount.into());
        }

        transfer_tokens(
            &self.maker_ata_x,
            &self.vault_x,
            &self.mint_x,
            &self.token_program,
            required_x,
        )?;

        transfer_tokens(
            &self.maker_ata_y,
            &self.vault_y,
            &self.mint_y,
            &self.token_program,
            required_y,
        )?;

        let signer_seeds: &[&[&[u8]]] =
            &[&[LP_SEED, &self.pool.key().to_bytes(), &[self.pool.lp_bump]]];

        mint_lp(
            &self.mint_lp,
            &self.maker_ata_lp,
            &self.pool,
            amount_lp,
            signer_seeds,
            &self.token_program,
        )?;

        Ok(())
    }
}
