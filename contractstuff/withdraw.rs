use anchor_lang::prelude::*;
use crate::state::UserEscrow;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// The user withdrawing SOL — must be the escrow owner
    #[account(mut)]
    pub user: Signer<'info>,

    /// The user's escrow account — PDA seeds enforce user ownership
    #[account(
        mut,
        seeds = [b"escrow", user.key().as_ref()],
        bump = escrow.bump,
        constraint = escrow.owner == user.key() @ WithdrawError::NotOwner,
    )]
    pub escrow: Account<'info, UserEscrow>,

    pub system_program: Program<'info, System>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, WithdrawError::ZeroAmount);

    let escrow = &mut ctx.accounts.escrow;

    require!(
        amount <= escrow.balance,
        WithdrawError::InsufficientBalance
    );

    // Decrease escrow balance
    escrow.balance = escrow.balance.checked_sub(amount).unwrap();

    // Transfer SOL from escrow PDA → user wallet
    // For PDA-owned accounts, we directly modify lamports
    **escrow.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

    msg!(
        "Withdrew {} lamports. Remaining balance: {}, locked: {}",
        amount,
        escrow.balance,
        escrow.locked,
    );

    Ok(())
}

#[error_code]
pub enum WithdrawError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Insufficient unlocked balance")]
    InsufficientBalance,
    #[msg("Only the escrow owner can withdraw")]
    NotOwner,
}
