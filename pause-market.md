# pause_market

Toggles a market between Open and Paused states. When paused, trading (buy/sell) is halted. Only the market's authority (admin) can call this.

**Caller:** Admin

## Parameters

None.

## Accounts

| Account | Type | Description |
|---|---|---|
| authority | Signer | Admin wallet (must match `market.authority`) |
| market | Account, mut | The market to pause/unpause |

## Logic

1. Validate caller is the market authority
2. If `Open` → set to `Paused`
3. If `Paused` → set to `Open`
4. If `Resolved` → error

## Errors

| Error | Condition |
|---|---|
| UnauthorizedAuthority | Signer is not the market authority |
| MarketAlreadyResolved | Market is already resolved (cannot toggle) |
