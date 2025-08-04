use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub const POOL_SEED: &[u8] = b"pool";
pub const LP_SEED: &[u8] = b"lp";
