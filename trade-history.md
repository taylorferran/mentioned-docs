# Trade History

How trade events are indexed and displayed.

## Overview

All instructions emit Anchor events. `buy` and `sell` emit `TradeEvent` logs which are captured by a Helius webhook, stored in Postgres, and served via REST API. The frontend queries the indexer instead of parsing raw on-chain transaction logs.

See [Indexer](indexer.md) for the full pipeline architecture, API endpoints, and database schema.

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
fetch('/api/trades?marketId=X&limit=50')
    │  → single HTTP call to the indexer API
    ▼
Returns trades from Postgres, newest-first
    │
    ▼
Map to TradeHistoryPoint[]
```

Previously this required `getSignaturesForAddress` + batch `getTransaction` + log parsing (5-15 RPC calls). Now it's one HTTP request.

### Chart data

For the price chart specifically, the frontend calls the dedicated chart endpoint:

```
fetch('/api/trades/chart?marketId=X&wordIndex=0&limit=500')
```

Returns trades oldest-first with `impliedPrice` ready for charting. Chart starts at 0.50 (LMSR initial price) and plots each trade's resulting implied YES price.

## Fetching user trade history

`fetchUserTradeHistory(userAddr, limit)` retrieves all trades by a specific user across all markets.

### How it works

```
fetch('/api/trades?trader=X&limit=50')
    │  → single HTTP call to the indexer API
    ▼
Returns trades with isBuy already determined server-side
    │
    ▼
fetchAllMarkets()
    │  → load markets for label resolution
    ▼
Enrich with market/word labels
    │
    ▼
Return UserTradeEntry[]
```

### Buy vs sell detection

Buy/sell is determined server-side by the webhook handler during insertion. Events are sorted chronologically, and for each event the global quantity for that word is compared before and after — if the relevant side's quantity increased, it's a buy.

### Where it's displayed

- **Profile page → History tab**: Full list with Buy (green) / Sell (orange) badges, timestamp, market/word labels, direction, quantity, cost, and Solana Explorer links. Sells show green `+` prefix on cost (money received), buys show `-` (money spent)
- **Profile page → Cost basis**: Trade history feeds the `costBasisMap` computation (see [Portfolio](portfolio.md))

## Pagination

The indexer API supports cursor-based pagination:

```
GET /api/trades?marketId=2&limit=50&before=2026-02-17T00:00:00Z
```

The `cursor` field in the response is the timestamp of the last result. Pass it as `before` for the next page.

## File references

- **Indexer pipeline**: See [Indexer](indexer.md)
- **Frontend SDK**: `lib/mentionMarket.ts` (`fetchTradeHistory`, `fetchUserTradeHistory`)
- **Market page chart**: `app/market/[id]/page.tsx`
- **Profile history tab**: `app/profile/page.tsx`
