use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::WordMarket;

#[derive(Accounts)]
pub struct Claim<'info> {
    /// The user claiming their winnings
    #[account(mut)]
    pub user: Signer<'info>,

    /// The resolved word market
    #[account(
        mut,
        // constraint: must be Resolved status
    )]
    pub word_market: Account<'info, WordMarket>,

    /// User's YES token account
    #[account(
        mut,
        token::mint = yes_mint,
        token::authority = user,
    )]
    pub user_yes_account: Account<'info, TokenAccount>,

    /// User's NO token account
    #[account(
        mut,
        token::mint = no_mint,
        token::authority = user,
    )]
    pub user_no_account: Account<'info, TokenAccount>,

    /// YES token mint
    #[account(
        mut,
        constraint = yes_mint.key() == word_market.yes_mint,
    )]
    pub yes_mint: Account<'info, Mint>,

    /// NO token mint
    #[account(
        mut,
        constraint = no_mint.key() == word_market.no_mint,
    )]
    pub no_mint: Account<'info, Mint>,

    /// CHECK: SOL vault PDA
    #[account(
        mut,
        seeds = [b"vault", word_market.key().as_ref()],
        bump = word_market.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Claim winnings from a resolved market.
///
/// Flow:
///   1. Check market is Resolved
///   2. Check which side won
///   3. Read user's winning token balance
///   4. Burn the winning tokens
///   5. Transfer equivalent SOL from vault → user wallet (1 token = 1 SOL)
///   6. Decrement word_market.total_collateral
pub fn handle_claim(_ctx: Context<Claim>) -> Result<()> {
    // TODO: Implement claim logic
    //
    // Steps:
    // 1. Verify market.status == Resolved and market.outcome.is_some()
    // 2. Determine winning mint based on outcome
    // 3. Get user's balance of winning token
    // 4. Burn user's winning tokens (CPI to token::burn)
    // 5. Transfer SOL from vault PDA → user wallet
    //    (vault PDA signs with seeds [b"vault", word_market.key(), &[vault_bump]])
    // 6. Decrement total_collateral
    //
    // The math is always solvent because:
    //   total_collateral = total YES minted = total NO minted
    //   Winning tokens ≤ total minted for that side
    //   So vault always has enough SOL

    msg!("claim: not yet implemented");
    err!(ClaimError::NotImplemented)
}

#[error_code]
pub enum ClaimError {
    #[msg("Not yet implemented")]
    NotImplemented,
    #[msg("Market is not resolved")]
    MarketNotResolved,
    #[msg("No winning tokens to claim")]
    NothingToClaim,
}
