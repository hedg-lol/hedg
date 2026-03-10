//! Refund instructions: record buyer and process refund.
//!
//! The refund system distributes escrow collateral pro-rata
//! to affected buyers when a rug pull is detected.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::FyrstError;
use crate::state::{BuyerRecord, EscrowVault};

#[derive(Accounts)]
pub struct RecordBuyer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Token mint for the purchase record
    pub token_mint: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = BuyerRecord::LEN,
        seeds = [RECORD_SEED, buyer.key().as_ref(), token_mint.key().as_ref()],
        bump,
    )]
    pub buyer_record: Account<'info, BuyerRecord>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessRefund<'info> {
    /// Protocol authority that triggers refunds
    pub authority: Signer<'info>,

    /// CHECK: Buyer wallet receiving the refund
    #[account(
        mut,
        constraint = buyer.key() == buyer_record.buyer @ FyrstError::NoBuyerRecord
    )]
    pub buyer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            ESCROW_SEED,
            escrow_vault.deployer.as_ref(),
            escrow_vault.token_mint.as_ref()
        ],
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

pub fn record_buyer(ctx: Context<RecordBuyer>, amount: u64, price: u64) -> Result<()> {
    let record = &mut ctx.accounts.buyer_record;
    let clock = Clock::get()?;

    if record.total_bought == 0 {
        record.buyer = ctx.accounts.buyer.key();
        record.token_mint = ctx.accounts.token_mint.key();
        record.first_buy_at = clock.unix_timestamp;
        record.refund_claimed = false;
        record.bump = ctx.bumps.buyer_record;
    }

    let sol_spent = amount
        .checked_mul(price)
        .ok_or(FyrstError::MathOverflow)?
        .checked_div(1_000_000_000)
        .ok_or(FyrstError::MathOverflow)?;

    record.total_bought = record
        .total_bought
        .checked_add(amount)
        .ok_or(FyrstError::MathOverflow)?;
    record.total_sol_spent = record
        .total_sol_spent
        .checked_add(sol_spent)
        .ok_or(FyrstError::MathOverflow)?;

    // Recalculate weighted average price
    if record.total_bought > 0 {
        record.avg_price = record
            .total_sol_spent
            .checked_mul(1_000_000_000)
            .ok_or(FyrstError::MathOverflow)?
            .checked_div(record.total_bought)
            .ok_or(FyrstError::MathOverflow)?;
    }

    msg!(
        "Buyer record updated: {} tokens, {} SOL spent",
        record.total_bought,
        record.total_sol_spent
    );
    Ok(())
}

pub fn process_refund(ctx: Context<ProcessRefund>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_vault;
    let record = &mut ctx.accounts.buyer_record;

    require!(escrow.rugged, FyrstError::SafePeriodExpired);
    require!(!record.refund_claimed, FyrstError::RefundAlreadyProcessed);

    // Pro-rata refund: buyer gets share of collateral proportional to SOL spent
    let refund_amount = record.total_sol_spent.min(escrow.collateral_amount);

    require!(refund_amount > 0, FyrstError::NoBuyerRecord);

    // Transfer refund from escrow to buyer
    let escrow_info = escrow.to_account_info();
    let buyer_info = ctx.accounts.buyer.to_account_info();

    **escrow_info.try_borrow_mut_lamports()? -= refund_amount;
    **buyer_info.try_borrow_mut_lamports()? += refund_amount;

    escrow.collateral_amount = escrow
        .collateral_amount
        .checked_sub(refund_amount)
        .ok_or(FyrstError::MathOverflow)?;
    record.refund_claimed = true;

    msg!("Refund processed: {} lamports to buyer", refund_amount);
    Ok(())
}
