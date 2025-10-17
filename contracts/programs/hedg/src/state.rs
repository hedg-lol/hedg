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
