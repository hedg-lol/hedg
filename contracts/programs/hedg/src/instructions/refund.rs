use anchor_lang::prelude::*;

use crate::state::BuyerRecord;
use crate::constants::*;
use crate::errors::HedgError;

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
        .ok_or(HedgError::MathOverflow)?
        .checked_div(1_000_000_000)
        .ok_or(HedgError::MathOverflow)?;

    record.total_bought = record.total_bought
        .checked_add(amount)
        .ok_or(HedgError::MathOverflow)?;
    record.total_sol_spent = record.total_sol_spent
        .checked_add(sol_spent)
        .ok_or(HedgError::MathOverflow)?;

    // Recalculate weighted average price
    if record.total_bought > 0 {
        record.avg_price = record.total_sol_spent
            .checked_mul(1_000_000_000)
            .ok_or(HedgError::MathOverflow)?
            .checked_div(record.total_bought)
            .ok_or(HedgError::MathOverflow)?;
    }

    msg!("Buyer record updated: {} tokens, {} SOL spent", record.total_bought, record.total_sol_spent);
    Ok(())
}
