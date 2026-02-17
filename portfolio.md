# Portfolio & Cost Basis

How the profile page tracks positions, computes P&L, and handles claims.

## Overview

The profile page (`app/profile/page.tsx`) is a full portfolio dashboard showing escrow balance, net investment, total returns (including realized redemptions), and trade history with cost basis tracking.

## Portfolio summary

Four overview cards at the top:

| Card | Source |
|---|---|
| **Escrow Balance** | `fetchEscrow(wallet)` |
| **Net Invested** | Total buy costs minus sell returns |
| **Total Returns** | Current position value + realized redemption returns |
| **Total P&L** | Total Returns − Net Invested (with percentage) |

## Tab interface

| Tab | Contents |
|---|---|
| **Active** | Open/Paused market positions with live LMSR-derived values |
| **Claimable** | Resolved positions where the user holds winning tokens |
| **Resolved** | All resolved positions with Claimed/Won/Sold/Lost badges |
| **History** | Full trade history with Buy/Sell badges and Explorer links |

## Resolved position states

The Resolved tab distinguishes four states based on win/loss status and current token balance:

| Condition | Badge | Payout |
|---|---|---|
| Won + balance = 0 | **Claimed** (green) | Already redeemed: net shares * 1 SOL |
| Won + balance > 0 | **Won** (green) | Claimable: current shares * 1 SOL |
| Lost + balance = 0 | **Sold** (gray) | 0 — user sold before resolution |
| Lost + balance > 0 | **Lost** (red) | 0 — tokens are worthless |

## Cost basis tracking

### CostBasis type

```typescript
interface CostBasis {
  totalCost: number    // Net SOL spent (buys minus sells)
  totalShares: number  // Net shares held
}
```

### Computation

A `costBasisMap` is computed from the user's trade history, keyed by `marketId-wordIndex-direction`:

1. Fetch user trade history via `fetchUserTradeHistory(wallet)`
2. For each **buy** trade, add to `totalCost` and `totalShares`
3. For each **sell** trade, subtract from `totalCost` and `totalShares`

Sells correctly reduce the cost basis rather than inflating it.

### Per-position display

Each position card shows:

| Field | Calculation |
|---|---|
| **Shares** | Token balance / 1e9 |
| **Avg Price** | `costBasis.totalCost / costBasis.totalShares` |
| **Current Value** | `shares * lmsrImpliedPrice` (active) or `shares * 1.0` (won) |
| **P&L** | `currentValue - (shares * avgPrice)` |
| **P&L %** | `(P&L / invested) * 100` |

### P&L aggregation

Total P&L includes both unrealized and realized components:

- **Net Invested** = sum of buy costs − sum of sell returns
- **Realized returns** = SOL received from claimed/redeemed positions (`costBasis.totalShares * 1 SOL` per redeemed position)
- **Total Returns** = current position value + realized redemption returns
- **Total P&L** = Total Returns − Net Invested

## Trade history tab

The History tab shows all trades with:

- **Buy/Sell badges** — Buy in green, Sell in orange
- **Cost display** — Buys show `-0.50 SOL` (money spent), sells show `+0.30 SOL` (money received)
- **Solana Explorer links** per transaction signature
- Timestamp, market/word labels, direction, quantity, average price

### Buy vs sell detection

Buy/sell is determined by tracking global token quantities across all program trades (not just the user's):

1. `fetchUserTradeHistory` fetches ALL program trade events, not just the user's
2. Events are sorted chronologically
3. Global YES/NO quantities are tracked per word
4. For each user trade, the relevant quantity is compared to the previous state
5. If the quantity increased → **buy**; if decreased → **sell**

This replaced the previous approach where `isBuy` was hardcoded to `true`.

## Claim flow

The Claimable tab shows positions ready to redeem with green border highlights.

### Batch claiming

"Claim All" button:
1. Calls `fetchMarket(marketId)` for each market with claimable positions
2. Creates `createRedeemIx` for each winning position
3. Batches all instructions into a single transaction
4. Signs, sends, confirms, refreshes

### Individual claiming

Each claimable position card also has its own "Claim" button for single-position redemption.

### Display

- **Claim value**: shares at 1 SOL per token
- **Profit**: claim value minus cost basis

## Data loading

On mount, three fetches run in parallel:
1. `fetchUserPositions(wallet)` — token balances across all markets (including 0-balance for resolved)
2. `fetchEscrow(wallet)` — escrow SOL balance
3. `fetchUserTradeHistory(wallet)` — trade events for cost basis and history tab

## File references

- **Profile page**: `app/profile/page.tsx`
- **Position fetching**: `lib/mentionMarket.ts` (`fetchUserPositions`)
- **Trade history**: `lib/mentionMarket.ts` (`fetchUserTradeHistory`)
