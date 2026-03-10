//! Account validation contexts for FYRST instructions.
//!
//! Each context struct defines the accounts required by an instruction
//! and their constraints (signer, writable, PDA seeds, etc.).

use anchor_lang::prelude::*;
use crate::state::{BondingCurve, BuyerRecord, EscrowVault};
use crate::constants::{ESCROW_SEED, CURVE_SEED, RECORD_SEED};

/// Accounts required by `create_escrow`.
#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

    /// CHECK: Token mint is validated by seed derivation.
    pub token_mint: AccountInfo<'info>,

    #[account(
        init,
        payer = deployer,
        space = 8 + EscrowVault::LEN,
        seeds = [ESCROW_SEED, deployer.key().as_ref(), token_mint.key().as_ref()],
        bump,
    )]
    pub escrow_vault: Account<'info, EscrowVault>,

    pub system_program: Program<'info, System>,
}

/// Accounts required by `release_escrow`.
#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(
        mut,
        constraint = deployer.key() == escrow_vault.deployer @ crate::errors::FyrstError::Unauthorized,
    )]
    pub deployer: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, deployer.key().as_ref(), escrow_vault.token_mint.as_ref()],
        bump = escrow_vault.bump,
    )]
    pub escrow_vault: Account<'info, EscrowVault>,

    pub system_program: Program<'info, System>,
}

/// Accounts required by `init_bonding_curve`.
#[derive(Accounts)]
pub struct InitBondingCurve<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

    /// CHECK: Token mint is validated by seed derivation.
    pub token_mint: AccountInfo<'info>,

    #[account(
        init,
        payer = deployer,
        space = 8 + BondingCurve::LEN,
        seeds = [CURVE_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub system_program: Program<'info, System>,
}

/// Accounts required by `buy_tokens`.
#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [CURVE_SEED, bonding_curve.token_mint.as_ref()],
        bump = bonding_curve.bump,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub system_program: Program<'info, System>,
}

/// Accounts required by `sell_tokens`.
#[derive(Accounts)]
pub struct SellTokens<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [CURVE_SEED, bonding_curve.token_mint.as_ref()],
        bump = bonding_curve.bump,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub system_program: Program<'info, System>,
}

/// Accounts required by `record_buyer`.
#[derive(Accounts)]
pub struct RecordBuyer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Token mint is validated by seed derivation.
    pub token_mint: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + BuyerRecord::LEN,
        seeds = [RECORD_SEED, buyer.key().as_ref(), token_mint.key().as_ref()],
        bump,
    )]
    pub buyer_record: Account<'info, BuyerRecord>,

    pub system_program: Program<'info, System>,
}

/// Accounts required by `process_refund`.
#[derive(Accounts)]
pub struct ProcessRefund<'info> {
    /// Protocol authority that triggers refunds.
    pub authority: Signer<'info>,

    /// CHECK: Buyer account receives the refund SOL.
    #[account(mut, constraint = buyer.key() == buyer_record.buyer)]
    pub buyer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow_vault.deployer.as_ref(), escrow_vault.token_mint.as_ref()],
        bump = escrow_vault.bump,
    )]
    pub escrow_vault: Account<'info, EscrowVault>,

    #[account(
        mut,
        seeds = [RECORD_SEED, buyer.key().as_ref(), escrow_vault.token_mint.as_ref()],
        bump = buyer_record.bump,
    )]
    pub buyer_record: Account<'info, BuyerRecord>,

    pub system_program: Program<'info, System>,
}
