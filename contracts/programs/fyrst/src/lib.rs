//! FYRST Protocol - Responsible Token Launchpad
//!
//! This program implements deployer collateral escrow,
//! bonding curve AMM, and pro-rata refund distribution.

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CcyByKGzRDK17icyNGAgdUN4q7WzbL1BPi4BNzqytyMP");

#[program]
pub mod fyrst {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, collateral_amount: u64) -> Result<()> {
        instructions::escrow::create_escrow(ctx, collateral_amount)
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
        instructions::escrow::release_escrow(ctx)
    }

    pub fn init_bonding_curve(
        ctx: Context<InitBondingCurve>,
        base_price: u64,
        slope: u64,
    ) -> Result<()> {
        instructions::bonding_curve::init_bonding_curve(ctx, base_price, slope)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        instructions::bonding_curve::buy_tokens(ctx, sol_amount)
    }

    pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {
        instructions::bonding_curve::sell_tokens(ctx, token_amount)
    }

    pub fn record_buyer(ctx: Context<RecordBuyer>, amount: u64, price: u64) -> Result<()> {
        instructions::refund::record_buyer(ctx, amount, price)
    }

    pub fn process_refund(ctx: Context<ProcessRefund>) -> Result<()> {
        instructions::refund::process_refund(ctx)
    }
}
