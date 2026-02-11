# create_market

Creates a new word market with YES and NO SPL token mints and a SOL vault. Each word within a market group gets its own `WordMarket` account.

**Caller:** Admin
**Status:** Implemented

## Parameters

| Name | Type | Description |
|---|---|---|
| market_id | u64 | Identifier grouping words under one market |
| word_index | u16 | Index of this word within the market |
| label | String | The word to trade on (max 32 chars) |

## Accounts

| Account | Type | Description |
|---|---|---|
| authority | Signer, mut | Admin wallet (pays for account creation) |
| word_market | PDA, init | The new WordMarket account |
| yes_mint | PDA, init | SPL token mint for YES shares (6 decimals) |
| no_mint | PDA, init | SPL token mint for NO shares (6 decimals) |
| vault | PDA | SOL vault for this market's collateral |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |
| rent | Sysvar | Rent sysvar |

## PDA derivation

```
word_market:  ["market", market_id (LE u64), word_index (LE u16)]
yes_mint:     ["yes_mint", word_market.key()]
no_mint:      ["no_mint", word_market.key()]
vault:        ["vault", word_market.key()]
```

Mint authority for both YES and NO mints is the `word_market` PDA itself.

## Logic

1. Validate `label.len() <= 32`
2. Initialize `WordMarket` account with all fields
3. Set status to `Active`, outcome to `None`, collateral to `0`
4. Create YES and NO SPL mints (6 decimal places)
5. Derive and store vault PDA address

## Errors

| Error | Condition |
|---|---|
| LabelTooLong | Label exceeds 32 characters |

## Source

```rust
pub fn handle_create_market(
    ctx: Context<CreateMarket>,
    market_id: u64,
    word_index: u16,
    label: String,
) -> Result<()> {
    require!(label.len() <= 32, MentionMarketError::LabelTooLong);

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

    Ok(())
}
```
