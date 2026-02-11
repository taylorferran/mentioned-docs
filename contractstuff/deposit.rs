use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::UserEscrow;

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// The user depositing SOL
    #[account(mut)]
    pub user: Signer<'info>,

    /// The user's escrow account — created on first deposit
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserEscrow::SIZE,
        seeds = [b"escrow", user.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, UserEscrow>,

    pub system_program: Program<'info, System>,
}

pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::ZeroAmount);

    // Transfer SOL from user wallet → escrow PDA
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update escrow state
    let escrow = &mut ctx.accounts.escrow;
    escrow.owner = ctx.accounts.user.key();
    escrow.balance = escrow.balance.checked_add(amount).unwrap();
    escrow.bump = ctx.bumps.escrow;

    msg!(
        "Deposited {} lamports. New balance: {}",
        amount,
        escrow.balance
    );

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
}
