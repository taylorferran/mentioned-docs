# CLOB Requirements

> **Status: Future design document.** The current MVP uses an on-chain AMM with LMSR pricing. This document describes the planned off-chain CLOB that would complement or replace the AMM as a v2 pricing mechanism. The `mention-market` (old CLOB) contract exists but is not wired to the frontend — all active trading uses `mention-market-amm`.

What the off-chain order book needs to provide so the frontend can support real trading.

## Current state (pre-CLOB)

**On-chain (working):**
- `deposit` / `withdraw` — user escrow management
- `create_market` — creates WordMarket with YES/NO mints
- `settle_match` — backend matches a YES buyer + NO buyer, deducts from escrows, mints tokens, funds vault
- `claim` — user burns winning tokens, receives SOL from vault
- `pause_market` / `resolve_market` — market lifecycle

**What's missing:**
- No way for users to place orders
- No order matching — `settle_match` is called manually by backend
- No price discovery — everything is 0.50/0.50
- `UserEscrow.locked` field exists but is unused (always 0)

## Architecture: off-chain CLOB + on-chain settlement

```
┌──────────────────────────────────────────────────────────┐
│                        Frontend                          │
│  Place order ──► CLOB API                                │
│  View order book ◄── CLOB API (REST + WebSocket)         │
│  Cancel order ──► CLOB API                               │
│  Deposit/Withdraw ──► Solana (on-chain, direct)          │
│  View positions ◄── Solana (on-chain, token balances)    │
│  Claim winnings ──► Solana (on-chain, direct)            │
└──────────────────────────────────────────────────────────┘
                          │
                          ▼
┌──────────────────────────────────────────────────────────┐
│                     CLOB Backend                         │
│  Order book per word market                              │
│  Matching engine                                         │
│  On match → call settle_match on-chain                   │
│  Manage escrow locks (lock on order, unlock on cancel)   │
└──────────────────────────────────────────────────────────┘
                          │
                          ▼
┌──────────────────────────────────────────────────────────┐
│                    Solana Program                         │
│  settle_match(price, amount) — mint tokens, move SOL     │
│  lock_funds / unlock_funds — escrow balance ↔ locked     │
│  claim() — burn winning tokens, receive SOL              │
└──────────────────────────────────────────────────────────┘
```

The backend is already the co-signer for `settle_match`. The CLOB extends this role to also manage the order book and trigger settlement when orders cross.

---

## 1. Order placement

### What the frontend sends

```typescript
{
  marketId: number
  wordIndex: number
  side: 'YES' | 'NO'
  price: number             // 0.01 – 0.99 in SOL
  quantity: number           // number of shares (1 share = 1 token = 1 SOL payout if correct)
}
```

### What happens

1. Frontend calls CLOB API with the order
2. CLOB validates user has sufficient unlocked escrow balance:
   - YES order cost = `price * quantity`
   - NO order cost = `(1 - price) * quantity`
3. CLOB calls `lock_funds` on-chain to move funds from `balance` → `locked`
4. Order is added to the order book
5. CLOB returns order ID + confirmation

### What the frontend needs back

```typescript
{
  orderId: string
  status: 'open' | 'rejected'
  reason?: string
  price: number
  quantity: number
  filledQuantity: number     // 0 initially, may be >0 if immediately matched
  side: 'YES' | 'NO'
  createdAt: number
}
```

### Escrow locking

The `UserEscrow` account already has a `locked` field (currently unused). The contract needs new instructions:

- **`lock_funds(amount)`** — called by backend when order is placed. Moves `amount` from `balance` to `locked`.
- **`unlock_funds(amount)`** — called by backend when order is cancelled. Moves `amount` from `locked` back to `balance`.

`settle_match` should be updated to deduct from `locked` (not `balance`) since matched orders already had funds locked at placement time.

---

## 2. Order book data

### What the frontend needs

Per word market, the frontend needs to display the current order book and derive prices.

**REST endpoint** — initial load / polling fallback:

```
GET /api/orderbook/:marketId/:wordIndex
```

```typescript
{
  marketId: number
  wordIndex: number
  bids: Array<{ price: number; quantity: number }>   // sorted highest → lowest
  asks: Array<{ price: number; quantity: number }>   // sorted lowest → highest
  lastTradePrice: number | null
  lastTradeTime: number | null
}
```

**Notes:**
- `bids` = YES buy orders (users willing to pay X for YES)
- `asks` = YES sell orders (equivalent to NO buy orders at `1 - price`)
- Aggregated by price level (sum quantities at same price)

### Price derivation

| Source | Calculation |
|---|---|
| Best bid | Highest YES buy price |
| Best ask | Lowest YES sell price |
| Mid price | `(bestBid + bestAsk) / 2` |
| Last trade | Price of most recent `settle_match` |

The NO price is always `1 - yesPrice`.

---

## 3. User's open orders

```
GET /api/orders/:walletAddress
```

```typescript
Array<{
  orderId: string
  marketId: number
  wordIndex: number
  wordLabel: string
  side: 'YES' | 'NO'
  price: number
  quantity: number
  filledQuantity: number
  status: 'open' | 'partial' | 'filled' | 'cancelled'
  createdAt: number
  updatedAt: number
}>
```

### Where this is displayed

- **Market page trading panel** — open orders for the currently selected word
- **Profile page** — new "Orders" tab showing all open orders across all markets
- **Header** — optionally show open order count as a badge

---

## 4. Cancel orders

```
POST /api/orders/:orderId/cancel
```

### What happens

1. CLOB removes order from the book
2. CLOB calls `unlock_funds` on-chain to move funds from `locked` → `balance`
3. Returns updated order with `status: 'cancelled'`

---

## 5. Real-time updates

WebSocket connection for live updates. Polling (every 5-10s) is an acceptable fallback for v1.

| Event | Payload | Used for |
|---|---|---|
| `orderbook_update` | `{ marketId, wordIndex, bids, asks }` | Live order book display |
| `trade` | `{ marketId, wordIndex, price, quantity, timestamp }` | Last trade price, chart data |
| `order_update` | `{ orderId, status, filledQuantity }` | User's order status changes |
| `price_update` | `{ marketId, wordIndex, bestBid, bestAsk, lastPrice }` | Price display on market cards |

**Subscription pattern:**

```typescript
ws.send({ type: 'subscribe', channel: 'orderbook', marketId: 1, wordIndex: 0 })
ws.send({ type: 'subscribe', channel: 'orders', wallet: '<pubkey>' })
```

---

## 6. Price updates across the frontend

| Location | Currently | After CLOB |
|---|---|---|
| Market page — word list | `yesPrice: '0.50'` hardcoded | `lastTradePrice` or `midPrice` from CLOB |
| Market page — trading panel | Hardcoded 0.50 | Best bid/ask from order book |
| Market page — chart | Random generated data | Real trade history |
| Home page — market cards | Mock prices | Last trade prices from CLOB |
| Profile — position est. value | `shares * 0.50` placeholder | `shares * lastTradePrice` |
| Header — portfolio value | `shares * 0.50` placeholder | `shares * lastTradePrice` |

---

## 7. Trade history

```
GET /api/trades/:marketId/:wordIndex?limit=100
```

```typescript
Array<{
  price: number
  quantity: number
  timestamp: number
  txSignature?: string       // on-chain settle_match tx for verification
}>
```

Feeds the chart component and the profile "History" tab.

---

## 8. Contract changes needed

### New instructions

| Instruction | Signer | Purpose |
|---|---|---|
| `lock_funds(amount)` | Backend | Move `amount` from escrow `balance` → `locked` when order placed |
| `unlock_funds(amount)` | Backend | Move `amount` from escrow `locked` → `balance` when order cancelled |

### Modified instructions

| Instruction | Change |
|---|---|
| `settle_match` | Deduct from `locked` instead of `balance` |
| `withdraw` | Already only allows `balance` (not `locked`), verify enforced |

### Open question: cancel all on market pause

When a market is paused, should all open orders be cancelled and funds unlocked? The CLOB backend would handle this — when it detects a `pause_market` event, it cancels all open orders and calls `unlock_funds` for each.

---

## 9. API authentication

Orders need to be authenticated to prevent spoofing:

- User signs a message with their wallet (e.g., `"Place order: YES 0.35 x 10 on market 1 word 0"`)
- Frontend sends the signature + message + public key to the CLOB API
- CLOB verifies the ed25519 signature matches the public key
- This proves the user authorized the order without needing the private key

---

## 10. Frontend integration summary

### New API client needed

```typescript
// lib/clobApi.ts

placeOrder(params: PlaceOrderRequest): Promise<Order>
cancelOrder(orderId: string): Promise<Order>
getOrderBook(marketId: number, wordIndex: number): Promise<OrderBook>
getUserOrders(wallet: string): Promise<Order[]>
getTradeHistory(marketId: number, wordIndex: number): Promise<Trade[]>
getPrices(marketId: number): Promise<Map<number, PriceInfo>>
```

### Pages to update

| Page | Changes |
|---|---|
| `app/market/[id]/page.tsx` | Wire Buy/Sell → `placeOrder()`. Real order book. Open orders. Real prices. Real chart. |
| `app/profile/page.tsx` | Add "Orders" tab. Use real prices for position values. |
| `components/Header.tsx` | Use real prices for portfolio valuation. |
| `lib/mentionMarket.ts` | Update `estimatePositionValue()` to accept price param. |

---

## 11. Order matching logic

A YES buy at price `P` can match with a NO buy at price `1 - P` (or lower for the NO side).

- YES bids are sorted highest first
- NO bids are converted to YES asks: a NO bid at `0.40` = YES ask at `0.60`
- When best YES bid >= best YES ask, a trade occurs
- The settlement price is the price of the resting order (maker price)
- Backend calls `settle_match(price, amount)` with the matched price

**Example:**
- User A bids YES at 0.35 for 5 shares → locks `0.35 * 5 = 1.75 SOL`
- User B bids NO at 0.60 for 5 shares → locks `0.60 * 5 = 3.00 SOL`
- NO bid at 0.60 = YES ask at 0.40
- User A's YES bid 0.35 < YES ask 0.40 → no match
- User C bids YES at 0.42 for 3 shares → locks `0.42 * 3 = 1.26 SOL`
- YES bid 0.42 >= YES ask 0.40 → match 3 shares at 0.40 (maker price)
- Backend calls `settle_match(price=0.40, amount=3)`
  - YES cost: `0.40 * 3 = 1.20 SOL` from User C's locked escrow
  - NO cost: `0.60 * 3 = 1.80 SOL` from User B's locked escrow
  - Refund User C: `1.26 - 1.20 = 0.06 SOL` back to balance (price improvement)
  - Vault receives: `3.00 SOL` total collateral
  - User C gets 3 YES tokens, User B gets 3 NO tokens
