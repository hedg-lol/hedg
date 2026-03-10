use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::constants::*;
use crate::errors::FyrstError;
use crate::state::EscrowVault;

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

    /// CHECK: Token mint account for the launched token
    pub token_mint: AccountInfo<'info>,

    #[account(
        init,
        payer = deployer,
        space = EscrowVault::LEN,
        seeds = [ESCROW_SEED, deployer.key().as_ref(), token_mint.key().as_ref()],
        bump,
    )]
    pub escrow_vault: Account<'info, EscrowVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(
        mut,
        constraint = deployer.key() == escrow_vault.deployer @ FyrstError::Unauthorized
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

/// Create a new escrow vault. The PDA bump is stored in the account
/// for efficient re-derivation in subsequent instructions.
pub fn create_escrow(ctx: Context<CreateEscrow>, collateral_amount: u64) -> Result<()> {
    require!(
        collateral_amount >= MIN_COLLATERAL,
        FyrstError::InsufficientCollateral
    );

    // Transfer collateral from deployer to escrow PDA
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.deployer.to_account_info(),
                to: ctx.accounts.escrow_vault.to_account_info(),
            },
        ),
        collateral_amount,
    )?;

    let escrow = &mut ctx.accounts.escrow_vault;
    escrow.deployer = ctx.accounts.deployer.key();
    escrow.token_mint = ctx.accounts.token_mint.key();
    escrow.collateral_amount = collateral_amount;
    escrow.created_at = Clock::get()?.unix_timestamp;
    escrow.released = false;
    escrow.rugged = false;
    escrow.bump = ctx.bumps.escrow_vault;

    msg!("Escrow created: {} lamports collateral", collateral_amount);
    Ok(())
}

pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_vault;

    require!(!escrow.released, FyrstError::EscrowAlreadyReleased);

    let now = Clock::get()?.unix_timestamp;
    let elapsed = now
        .checked_sub(escrow.created_at)
        .ok_or(FyrstError::MathOverflow)?;
    // Ensure elapsed time is non-negative (clock can sometimes drift)
    require!(elapsed >= SAFE_PERIOD, FyrstError::SafePeriodActive);

    // Transfer collateral back to deployer
    let escrow_info = escrow.to_account_info();
    let deployer_info = ctx.accounts.deployer.to_account_info();
    let amount = escrow.collateral_amount;

    **escrow_info.try_borrow_mut_lamports()? -= amount;
    **deployer_info.try_borrow_mut_lamports()? += amount;

    escrow.released = true;

    msg!("Escrow released: {} lamports returned to deployer", amount);
    Ok(())
}
