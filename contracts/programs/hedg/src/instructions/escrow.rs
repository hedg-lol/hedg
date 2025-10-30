use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::EscrowVault;
use crate::constants::*;
use crate::errors::HedgError;

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

pub fn create_escrow(ctx: Context<CreateEscrow>, collateral_amount: u64) -> Result<()> {
    require!(
        collateral_amount >= MIN_COLLATERAL,
        HedgError::InsufficientCollateral
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
