use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use constant_product_curve::ConstantProduct;

use crate::{swap_tokens, Pool, POOL_SEED};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [POOL_SEED, mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn process_swap(&self, amount_x: u64, amount_y: u64, is_x: bool) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] =
            &[&[POOL_SEED, &self.pool.key().to_bytes(), &[self.pool.bump]]];

        match is_x {
            true => {
                let expected_x = ConstantProduct::delta_x_from_y_swap_amount(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    amount_y.checked_mul(1 - self.pool.swap_fee as u64).unwrap(),
                )
                .unwrap();
                swap_tokens(
                    &self.mint_x,
                    &self.mint_y,
                    &self.vault_x,
                    &self.vault_y,
                    &self.maker_ata_x,
                    &self.maker_ata_y,
                    expected_x,
                    amount_y,
                    true,
                    signer_seeds,
                    &self.token_program,
                )?;
            }
            false => {
                let expected_y = ConstantProduct::delta_y_from_x_swap_amount(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    amount_x.checked_mul(1 - self.pool.swap_fee as u64).unwrap(),
                )
                .unwrap();
                swap_tokens(
                    &self.mint_x,
                    &self.mint_y,
                    &self.vault_x,
                    &self.vault_y,
                    &self.maker_ata_x,
                    &self.maker_ata_y,
                    amount_x,
                    expected_y,
                    false,
                    signer_seeds,
                    &self.token_program,
                )?;
            }
        };

        Ok(())
    }
}
