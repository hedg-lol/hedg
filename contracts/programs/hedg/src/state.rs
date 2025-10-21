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
