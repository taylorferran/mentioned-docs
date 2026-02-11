use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::{WordMarket, MarketStatus};

#[derive(Accounts)]
#[instruction(market_id: u64, word_index: u16, label: String)]
pub struct CreateMarket<'info> {
    /// Admin authority creating the market
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The word market account to initialize
    #[account(
        init,
        payer = authority,
        space = 8 + WordMarket::SIZE,
        seeds = [b"market", market_id.to_le_bytes().as_ref(), word_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub word_market: Account<'info, WordMarket>,

    /// YES token mint — mint authority is the word_market PDA
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = word_market,
        seeds = [b"yes_mint", word_market.key().as_ref()],
        bump,
    )]
    pub yes_mint: Account<'info, Mint>,

    /// NO token mint — mint authority is the word_market PDA
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = word_market,
        seeds = [b"no_mint", word_market.key().as_ref()],
        bump,
    )]
    pub no_mint: Account<'info, Mint>,

    /// Vault PDA to hold collateral SOL
    /// CHECK: This is a PDA used as a SOL vault, no data needed
    #[account(
        seeds = [b"vault", word_market.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_market(
    ctx: Context<CreateMarket>,
    market_id: u64,
    word_index: u16,
    label: String,
) -> Result<()> {
    require!(label.len() <= 32, CreateMarketError::LabelTooLong);

    let word_market = &mut ctx.accounts.word_market;
    word_market.authority = ctx.accounts.authority.key();
    word_market.market_id = market_id;
    word_market.word_index = word_index;
    word_market.label = label;
    word_market.yes_mint = ctx.accounts.yes_mint.key();
    word_market.no_mint = ctx.accounts.no_mint.key();
    word_market.vault = ctx.accounts.vault.key();
    word_market.total_collateral = 0;
    word_market.status = MarketStatus::Active;
    word_market.outcome = None;
    word_market.bump = ctx.bumps.word_market;
    word_market.vault_bump = ctx.bumps.vault;

    msg!("Market created: {} (id={}, word={})", word_market.label, market_id, word_index);

    Ok(())
}

#[error_code]
pub enum CreateMarketError {
    #[msg("Label must be 32 characters or fewer")]
    LabelTooLong,
}
