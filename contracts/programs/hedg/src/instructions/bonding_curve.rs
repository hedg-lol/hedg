use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::BondingCurve;
use crate::constants::*;
use crate::errors::HedgError;

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

    msg!("Bonding curve initialized: base_price={}, slope={}", base_price, slope);
    Ok(())
}
