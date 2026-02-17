# Trade History

How trade events are fetched and displayed from on-chain transaction logs.

## Overview

All instructions now emit Anchor events. `buy` and `sell` emit `TradeEvent` logs, which the frontend parses from raw transaction data to build trade history charts and user activity feeds. Other events (EscrowEvent, MarketCreatedEvent, MarketPausedEvent, LiquidityEvent, ResolutionEvent, RedemptionEvent) are also emitted but not yet indexed client-side. Currently there is no indexer — events are fetched directly from the RPC. See [Indexer Plan](INDEXER_PLAN.md) for the full list of event types and the plan to capture them all server-side.

## Types

### TradeHistoryPoint

Represents a single trade's impact on a word's price. Used for chart rendering.

```typescript
interface TradeHistoryPoint {
  timestamp: number          // Unix timestamp
  wordIndex: number          // Which word was traded
  impliedYesPrice: number    // Resulting YES price (0..1)
  direction: 'YES' | 'NO'   // Side traded
  quantity: number           // Shares traded
  cost: number               // SOL cost/return
}
```

### UserTradeEntry

A trade from a specific user's perspective. Used for the profile history tab.

```typescript
interface UserTradeEntry {
  timestamp: number
  marketId: bigint
  marketLabel: string
  wordIndex: number
  wordLabel: string
  direction: 'YES' | 'NO'
  quantity: number           // Shares
  cost: number               // SOL
  isBuy: boolean
  txSignature: string        // For Explorer links
}
```

## Fetching market trade history

`fetchTradeHistory(marketId, limit)` retrieves trade events for a specific market.

### How it works

```
getSignaturesForAddress(marketPDA, { limit })
    │  → returns transaction signatures touching the market account
    ▼
Batch fetch transactions (10 at a time)
    │  → getTransaction(signature) for each
    ▼
Parse log messages
    │  → scan for "Program data: " prefixes
    │  → base64-decode the payload
    │  → match Anchor event discriminator for TradeEvent
    ▼
Decode event fields
    │  → trader, marketId, wordIndex, side, quantity, cost, timestamp
    ▼
Compute implied price after each trade
    │  → lmsrImpliedPrice(newYesQty, newNoQty, b)
    ▼
Return TradeHistoryPoint[]
```

### Chart rendering

Trade history feeds the price chart on the market page:

- Starts at 0.50 (LMSR initial price when quantities are equal)
- Each trade point plots the resulting implied YES price for that word
- Per-word price lines are shown (one line per word in the market)

## Fetching user trade history

`fetchUserTradeHistory(userAddr, limit)` retrieves all trades by a specific user across all markets.

### How it works

```
getSignaturesForAddress(PROGRAM_ID, { limit })
    │  → scans the entire program's transaction history
    ▼
Batch fetch + parse ALL TradeEvent logs (not just the user's)
    │
    ▼
Sort chronologically, track global YES/NO quantities per word
    │
    ▼
For each event where trader == userAddr:
    │  → compare quantity to previous global state
    │  → quantity increased = buy, decreased = sell
    ▼
fetchAllMarkets()
    │  → load all markets for label resolution
    ▼
Enrich with market/word labels, isBuy flag
    │
    ▼
Return UserTradeEntry[]
```

### Buy vs sell detection

The `isBuy` flag is determined by global quantity tracking rather than any field on the event itself:

1. All program trade events are collected and sorted by timestamp
2. Running totals of `yesQuantity` and `noQuantity` are maintained per `(marketId, wordIndex)`
3. For each user trade, the `newYesQty` / `newNoQty` from the event is compared to the running total
4. If the relevant quantity went up → **buy**; if it went down → **sell**

This is necessary because the `TradeEvent` doesn't contain an explicit buy/sell flag.

### Where it's displayed

- **Profile page → History tab**: Full list with Buy (green) / Sell (orange) badges, timestamp, market/word labels, direction, quantity, cost, and Solana Explorer links. Sells show green `+` prefix on cost (money received), buys show `-` (money spent)
- **Profile page → Cost basis**: Trade history feeds the `costBasisMap` computation (see [Portfolio](portfolio.md))

## Limitations

This approach works for the devnet prototype but has scaling issues:

- **`fetchTradeHistory`**: Fetches full transactions in batches of 10, parsing every log line — slow for markets with many trades
- **`fetchUserTradeHistory`**: Scans the *entire program's* transaction history and filters client-side — unusable at scale
- **`getSignaturesForAddress`** has a default limit (1000) and will miss older trades as history grows

See [Scalability Roadmap](scalability.md) for the planned migration to an indexer.

## File references

- **Source**: `lib/mentionMarket.ts` (`fetchTradeHistory`, `fetchUserTradeHistory`)
- **Market page chart**: `app/market/[id]/page.tsx`
- **Profile history tab**: `app/profile/page.tsx`
