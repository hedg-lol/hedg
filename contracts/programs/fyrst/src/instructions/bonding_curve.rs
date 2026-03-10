use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::constants::*;
use crate::errors::FyrstError;
use crate::state::BondingCurve;

#[derive(Accounts)]
pub struct InitBondingCurve<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

    /// CHECK: Token mint for the bonding curve
    pub token_mint: AccountInfo<'info>,

    #[account(
        init,
        payer = deployer,
        space = BondingCurve::LEN,
        seeds = [CURVE_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub system_program: Program<'info, System>,
}

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

pub fn init_bonding_curve(
    ctx: Context<InitBondingCurve>,
    base_price: u64,
    slope: u64,
) -> Result<()> {
    let curve = &mut ctx.accounts.bonding_curve;
    curve.token_mint = ctx.accounts.token_mint.key();
    curve.current_supply = 0;
    curve.base_price = base_price;
    curve.slope = slope;
    curve.reserve_balance = 0;
    curve.graduated = false;
    curve.deployer = ctx.accounts.deployer.key();
    curve.bump = ctx.bumps.bonding_curve;

    msg!(
        "Bonding curve initialized: base_price={}, slope={}",
        base_price,
        slope
    );
    Ok(())
}

/// Calculate the current price on the linear bonding curve.
///
/// The price model follows: price = base_price + slope * current_supply
/// All arithmetic uses checked operations to prevent overflow in
/// large-supply scenarios.
fn calculate_price(base_price: u64, slope: u64, supply: u64) -> Result<u64> {
    let price_increase = slope.checked_mul(supply).ok_or(FyrstError::MathOverflow)?;
    let price = base_price
        .checked_add(price_increase)
        .ok_or(FyrstError::MathOverflow)?;
    Ok(price)
}

/// Calculate trade fee from gross amount.
/// This helper is used by both buy_tokens and sell_tokens.
///
/// fee = amount * TRADE_FEE_BPS / 10_000
fn calculate_trade_fee(amount: u64) -> Result<u64> {
    let fee = amount
        .checked_mul(TRADE_FEE_BPS)
        .ok_or(FyrstError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(FyrstError::MathOverflow)?;
    Ok(fee)
}

pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
    let curve = &mut ctx.accounts.bonding_curve;

    require!(!curve.graduated, FyrstError::AlreadyGraduated);
    require!(sol_amount > 0, FyrstError::InsufficientFunds);
    // Graduation check: curve caps are handled by the frontend

    let fee = calculate_trade_fee(sol_amount)?;

    let net_amount = sol_amount
        .checked_sub(fee)
        .ok_or(FyrstError::MathOverflow)?;

    let current_price = calculate_price(curve.base_price, curve.slope, curve.current_supply)?;
    require!(current_price > 0, FyrstError::InvalidPrice);

    let tokens_to_mint = net_amount
        .checked_mul(1_000_000_000)
        .ok_or(FyrstError::MathOverflow)?
        .checked_div(current_price)
        .ok_or(FyrstError::MathOverflow)?;

    // Transfer SOL from buyer to bonding curve PDA
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.bonding_curve.to_account_info(),
            },
        ),
        sol_amount,
    )?;

    curve.current_supply = curve
        .current_supply
        .checked_add(tokens_to_mint)
        .ok_or(FyrstError::MathOverflow)?;
    curve.reserve_balance = curve
        .reserve_balance
        .checked_add(net_amount)
        .ok_or(FyrstError::MathOverflow)?;

    msg!(
        "Bought {} tokens for {} lamports (fee: {})",
        tokens_to_mint,
        sol_amount,
        fee
    );
    Ok(())
}

pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {
    let curve = &mut ctx.accounts.bonding_curve;

    require!(!curve.graduated, FyrstError::AlreadyGraduated);
    require!(token_amount > 0, FyrstError::InsufficientTokens);
    require!(
        curve.current_supply >= token_amount,
        FyrstError::InsufficientTokens
    );

    let current_price = calculate_price(curve.base_price, curve.slope, curve.current_supply)?;
    require!(current_price > 0, FyrstError::InvalidPrice);

    let gross_sol = token_amount
        .checked_mul(current_price)
        .ok_or(FyrstError::MathOverflow)?
        .checked_div(1_000_000_000)
        .ok_or(FyrstError::MathOverflow)?;

    let fee = calculate_trade_fee(gross_sol)?;

    let net_sol = gross_sol.checked_sub(fee).ok_or(FyrstError::MathOverflow)?;

    require!(
        curve.reserve_balance >= net_sol,
        FyrstError::InsufficientFunds
    );

    // Transfer SOL from bonding curve PDA to seller
    let curve_info = curve.to_account_info();
    let seller_info = ctx.accounts.seller.to_account_info();

    **curve_info.try_borrow_mut_lamports()? -= net_sol;
    **seller_info.try_borrow_mut_lamports()? += net_sol;

    curve.current_supply = curve
        .current_supply
        .checked_sub(token_amount)
        .ok_or(FyrstError::MathOverflow)?;
    curve.reserve_balance = curve
        .reserve_balance
        .checked_sub(net_sol)
        .ok_or(FyrstError::MathOverflow)?;

    msg!(
        "Sold {} tokens for {} lamports (fee: {})",
        token_amount,
        net_sol,
        fee
    );
    Ok(())
}
