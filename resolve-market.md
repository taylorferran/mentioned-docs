# resolve_market

Resolves a market with a YES or NO outcome. Can be called on markets that are Active or Paused. Once resolved, users can claim winnings.

**Caller:** Admin
**Status:** Implemented

## Parameters

| Name | Type | Description |
|---|---|---|
| outcome | Outcome | `Yes` or `No` |

## Accounts

| Account | Type | Description |
|---|---|---|
| authority | Signer | Admin wallet (must match `word_market.authority`) |
| word_market | Account, mut | The market to resolve |

## Logic

1. Validate caller is the market authority
2. Validate market is not already `Resolved`
3. Set status to `Resolved`
4. Store the outcome (`Yes` or `No`)

## Errors

| Error | Condition |
|---|---|
| UnauthorizedAuthority | Signer is not the market authority |
| MarketAlreadyResolved | Market has already been resolved |

## Source

```rust
pub fn handle_resolve_market(ctx: Context<ResolveMarket>, outcome: Outcome) -> Result<()> {
    let word_market = &mut ctx.accounts.word_market;

    require!(
        word_market.status != MarketStatus::Resolved,
        MentionMarketError::MarketAlreadyResolved
    );

    word_market.status = MarketStatus::Resolved;
    word_market.outcome = Some(outcome);

    Ok(())
}
```
