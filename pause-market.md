# pause_market

Pauses an active market, halting trading. Only the market's authority (admin) can call this.

**Caller:** Admin
**Status:** Implemented

## Parameters

None.

## Accounts

| Account | Type | Description |
|---|---|---|
| authority | Signer | Admin wallet (must match `word_market.authority`) |
| word_market | Account, mut | The market to pause |

## Logic

1. Validate caller is the market authority
2. Validate market status is `Active`
3. Set status to `Paused`

## Errors

| Error | Condition |
|---|---|
| UnauthorizedAuthority | Signer is not the market authority |
| MarketNotActive | Market is not in `Active` status |

## Source

```rust
pub fn handle_pause_market(ctx: Context<PauseMarket>) -> Result<()> {
    let word_market = &mut ctx.accounts.word_market;

    require!(
        word_market.status == MarketStatus::Active,
        MentionMarketError::MarketNotActive
    );

    word_market.status = MarketStatus::Paused;

    Ok(())
}
```
