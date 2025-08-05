use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    burn, mint_to, transfer_checked, Burn, Mint, MintTo, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::Pool;

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    token_program: &Interface<'info, TokenInterface>,
    amount: u64,
) -> Result<()> {
    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: from.to_account_info(),
                to: to.to_account_info(),
                mint: mint.to_account_info(),
                authority: from.to_account_info(),
            },
        ),
        amount,
        mint.decimals,
    )?;
    Ok(())
}

pub fn transfer_tokens_pda<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: from.to_account_info(),
                to: to.to_account_info(),
                mint: mint.to_account_info(),
                authority: from.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        mint.decimals,
    )?;
    Ok(())
}

pub fn mint_lp<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    authority: &Account<'info, Pool>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                authority: authority.to_account_info(),
                mint: mint.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}

pub fn burn_lp<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    from: &InterfaceAccount<'info, TokenAccount>,
    authority: &Account<'info, Pool>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    burn(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Burn {
                mint: mint.to_account_info(),
                from: from.to_account_info(),
                authority: authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}
