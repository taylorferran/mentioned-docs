# Free Markets

Free markets are virtual prediction markets using play tokens — no real money. Users trade YES/NO shares on word outcomes using an on-chain-style LMSR AMM running in PostgreSQL. Wins earn points.

## Overview

| Property | Value |
|----------|-------|
| Currency | Play tokens (no real value) |
| Pricing | LMSR AMM (virtual, all in PostgreSQL) |
| Creation | Admin only |
| Points earned | 50% of net token profit (if Discord linked) |

## Market Lifecycle

```
draft → open → locked → resolved
```

| Status | Description |
|--------|-------------|
| `draft` | Created by admin, not yet visible to users |
| `open` | Trading live. Users can buy/sell. Lock time is in the future. |
| `locked` | `lock_time` has passed. No new trades. Awaiting resolution. |
| `resolved` | All words resolved. Payouts computed. Points awarded. |

When a trade is attempted on an `open` market whose `lock_time` has passed, the market is atomically transitioned to `locked` before the trade is rejected.

## Trading

**Endpoint:** `POST /api/custom/[id]/trade`

**Parameters:**
| Field | Type | Description |
|-------|------|-------------|
| wallet | string | Trader's wallet |
| word_id | number | ID of the word to trade |
| action | `buy` \| `sell` | Trade direction |
| side | `YES` \| `NO` | Outcome side |
| amount | number | Amount to spend or shares to sell |
| amount_type | `tokens` \| `shares` | Whether `amount` is tokens or shares |

**Execution:**
1. Validate market is `open` and `lock_time` has not passed
2. Lock pool row with `FOR UPDATE` (atomic)
3. Calculate cost/return via LMSR pricing
4. Update `custom_market_word_pools` (yes_qty / no_qty)
5. Update `custom_market_positions` (yes_shares / no_shares, tokens_spent/received)
6. Update `custom_market_balances` (deduct/add tokens)
7. Insert into `custom_market_trades` log
8. Insert into `custom_market_price_history` for charting
9. Check and unlock achievements (`first_free_trade`)

Initial balance is set to the market's `play_tokens` value (default 1000) on first trade.

Rate limit: 500ms per wallet.

## LMSR Pricing

Virtual markets use Logarithmic Market Scoring Rule pricing from `lib/virtualLmsr.ts`.

**Cost function:**
```
C(q_yes, q_no) = b * ln(exp(q_yes / b) + exp(q_no / b))
```

**Key functions:**

| Function | Description |
|----------|-------------|
| `virtualBuyCost(pool, side, shares, b)` | Cost in tokens to buy N shares |
| `virtualSellReturn(pool, side, shares, b)` | Tokens returned for selling N shares |
| `sharesForTokens(pool, side, tokens, b)` | How many shares a token amount buys |
| `impliedPrice(pool, side, b)` | Current YES/NO implied probability |

The `b` parameter is set per market (default 500). Higher `b` = less price impact per trade = tighter spreads.

## Resolution

**Endpoint:** `POST /api/custom/[id]/resolve`

Admin only. Resolves all words and transitions the market to `resolved`.

**Request body:**
```json
{
  "wallet": "...",
  "outcomes": [
    { "word_id": 1, "outcome": true },
    { "word_id": 2, "outcome": false }
  ]
}
```

**Process:**
1. Validate admin and that market is `locked`
2. Atomic CAS transition: `locked → resolved`
3. Set `resolved_outcome` on each word in `custom_market_words`
4. Fire-and-forget call to `resolveAndScoreVirtualMarket`:
   - For each wallet with positions, compute `net = tokens_received - tokens_spent`
   - Award `max(0, floor(net * 0.5))` points via `custom_market_win` action
   - Unlock `free_market_win` achievement for winners

## Data Model

| Table | Purpose |
|-------|---------|
| `custom_markets` | Market metadata (title, status, b_parameter, play_tokens, lock_time) |
| `custom_market_words` | Words/outcomes in the market |
| `custom_market_word_pools` | LMSR pool state (yes_qty, no_qty per word) |
| `custom_market_positions` | User share holdings per word |
| `custom_market_balances` | Play token balance per user per market |
| `custom_market_trades` | Full trade log |
| `custom_market_price_history` | Price snapshots for charting |

See [Database Schema](database.md) for full column details.
