use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{WordMarket, UserEscrow};

#[derive(Accounts)]
pub struct SettleMatch<'info> {
    /// Backend co-signer — only your backend can call this
    pub backend: Signer<'info>,

    /// The word market being traded
    #[account(
        mut,
        // constraint: market must be active
    )]
    pub word_market: Account<'info, WordMarket>,

    // --- YES buyer side ---

    /// YES buyer's escrow
    #[account(
        mut,
        seeds = [b"escrow", yes_buyer.key().as_ref()],
        bump = yes_buyer_escrow.bump,
    )]
    pub yes_buyer_escrow: Account<'info, UserEscrow>,

    /// CHECK: YES buyer wallet, validated by escrow seeds
    pub yes_buyer: UncheckedAccount<'info>,

    /// YES buyer's token account for YES tokens
    #[account(
        mut,
        token::mint = yes_mint,
        token::authority = yes_buyer,
    )]
    pub yes_buyer_token_account: Account<'info, TokenAccount>,

    // --- NO buyer side ---

    /// NO buyer's escrow
    #[account(
        mut,
        seeds = [b"escrow", no_buyer.key().as_ref()],
        bump = no_buyer_escrow.bump,
    )]
    pub no_buyer_escrow: Account<'info, UserEscrow>,

    /// CHECK: NO buyer wallet, validated by escrow seeds
    pub no_buyer: UncheckedAccount<'info>,

    /// NO buyer's token account for NO tokens
    #[account(
        mut,
        token::mint = no_mint,
        token::authority = no_buyer,
    )]
    pub no_buyer_token_account: Account<'info, TokenAccount>,

    // --- Token mints ---

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

    // --- Vault ---

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

/// Settle a matched order pair.
///
/// `price` is in lamports per share (what the YES buyer pays).
/// The NO buyer pays (1_000_000_000 - price) per share (1 SOL = 1e9 lamports).
/// `amount` is the number of shares (in token base units, 1e6 = 1 share).
///
/// Flow:
///   1. Deduct (price × amount) from yes_buyer_escrow.locked
///   2. Deduct ((1 SOL - price) × amount) from no_buyer_escrow.locked
///   3. Transfer total SOL to vault
///   4. Mint `amount` YES tokens to yes_buyer
///   5. Mint `amount` NO tokens to no_buyer
///   6. Increment word_market.total_collateral
pub fn handle_settle_match(
    _ctx: Context<SettleMatch>,
    _price: u64,
    _amount: u64,
) -> Result<()> {
    // TODO: Implement settlement logic
    //
    // Steps:
    // 1. Verify market status == Active
    // 2. Calculate costs:
    //    - yes_cost = (price * amount) / SHARES_DECIMALS
    //    - no_cost = ((LAMPORTS_PER_SOL - price) * amount) / SHARES_DECIMALS
    // 3. Verify and deduct from escrows:
    //    - yes_buyer_escrow.locked >= yes_cost
    //    - no_buyer_escrow.locked >= no_cost
    // 4. Transfer SOL from escrows to vault
    // 5. Mint YES tokens to yes_buyer (CPI to token program, signed by word_market PDA)
    // 6. Mint NO tokens to no_buyer (CPI to token program, signed by word_market PDA)
    // 7. Update word_market.total_collateral

    msg!("settle_match: not yet implemented");
    err!(SettleMatchError::NotImplemented)
}

#[error_code]
pub enum SettleMatchError {
    #[msg("Not yet implemented")]
    NotImplemented,
    #[msg("Market is not active")]
    MarketNotActive,
    #[msg("Insufficient locked balance for YES buyer")]
    InsufficientYesFunds,
    #[msg("Insufficient locked balance for NO buyer")]
    InsufficientNoFunds,
    #[msg("Price must be between 0 and 1 SOL")]
    InvalidPrice,
}
