use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
use state::Outcome;

declare_id!("MENTionXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod mention_market {
    use super::*;

    // ============================================================
    // User instructions
    // ============================================================

    /// Deposit SOL into the user's escrow account.
    /// Creates the escrow on first deposit.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handle_deposit(ctx, amount)
    }

    /// Withdraw unlocked SOL from the user's escrow account.
    /// Cannot withdraw SOL that is locked in open orders.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handle_withdraw(ctx, amount)
    }

    /// Claim winnings from a resolved market.
    /// Burns the user's winning tokens and transfers SOL 1:1 from the vault.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim::handle_claim(ctx)
    }

    // ============================================================
    // Admin instructions
    // ============================================================

    /// Create a new word market with YES/NO token mints and a SOL vault.
    pub fn create_market(
        ctx: Context<CreateMarket>,
        market_id: u64,
        word_index: u16,
        label: String,
    ) -> Result<()> {
        instructions::create_market::handle_create_market(ctx, market_id, word_index, label)
    }

    /// Pause an active market — blocks new settlements.
    pub fn pause_market(ctx: Context<PauseMarket>) -> Result<()> {
        instructions::pause_market::handle_pause_market(ctx)
    }

    /// Resolve a market with a YES or NO outcome.
    /// After resolution, users can claim winnings.
    pub fn resolve_market(ctx: Context<ResolveMarket>, outcome: Outcome) -> Result<()> {
        instructions::resolve_market::handle_resolve_market(ctx, outcome)
    }

    // ============================================================
    // Backend instruction (co-signed by backend wallet)
    // ============================================================

    /// Settle a matched order pair.
    /// Called by the backend after matching a YES buyer with a NO buyer.
    /// Moves SOL from both escrows into the market vault and mints tokens.
    pub fn settle_match(
        ctx: Context<SettleMatch>,
        price: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::settle_match::handle_settle_match(ctx, price, amount)
    }
}
