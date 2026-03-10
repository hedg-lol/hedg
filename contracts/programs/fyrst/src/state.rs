//! Account state definitions for the FYRST protocol.
//!
//! Discriminator: 8 bytes prepended by Anchor to each account.
//! All LEN calculations include the discriminator.

use anchor_lang::prelude::*;

#[account]
pub struct EscrowVault {
    pub deployer: Pubkey,
    pub token_mint: Pubkey,
    pub collateral_amount: u64,
    pub created_at: i64,
    pub released: bool,
    pub rugged: bool,
    pub bump: u8,
}

impl EscrowVault {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1 + 1;
}

#[account]
pub struct BondingCurve {
    pub token_mint: Pubkey,
    pub current_supply: u64,
    pub base_price: u64,
    pub slope: u64,
    pub reserve_balance: u64,
    pub graduated: bool,
    pub deployer: Pubkey,
    pub bump: u8,
}

impl BondingCurve {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 1 + 32 + 1;
}

#[account]
pub struct BuyerRecord {
    pub buyer: Pubkey,
    pub token_mint: Pubkey,
    pub total_bought: u64,
    pub total_sol_spent: u64,
    pub avg_price: u64,
    pub refund_claimed: bool,
    pub first_buy_at: i64,
    pub bump: u8,
}

impl BuyerRecord {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 8 + 1;
}
