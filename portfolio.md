# Portfolio & Cost Basis

How the profile page tracks positions, computes P&L, and handles claims.

## Overview

The profile page (`app/profile/page.tsx`) is a full portfolio dashboard showing escrow balance, active positions, claimable winnings, and trade history with cost basis tracking.

## Portfolio summary

Four overview cards at the top:

| Card | Source |
|---|---|
| **Escrow Balance** | `fetchEscrow(wallet)` |
| **Total Invested** | Sum of all buy costs from trade history |
| **Current Value** | Sum of `estimatedValueSol` across all positions |
| **Total P&L** | Current Value − Total Invested (with percentage) |

## Tab interface

| Tab | Contents |
|---|---|
| **Active** | Open/Paused market positions with live LMSR-derived values |
| **Claimable** | Resolved positions where the user holds winning tokens |
| **Resolved** | All resolved positions with Won/Lost badges |
| **History** | Full trade history from on-chain event logs |

## Cost basis tracking

### CostBasis type

```typescript
interface CostBasis {
  totalCost: number    // Total SOL spent on buys
  totalShares: number  // Total shares acquired
}
```

### Computation

A `costBasisMap` is computed from the user's trade history, keyed by `marketId-wordIndex-direction`:

1. Fetch user trade history via `fetchUserTradeHistory(wallet)`
2. For each buy trade, add to `totalCost` and `totalShares`
3. For each sell trade, subtract proportionally

### Per-position display

Each position card shows:

| Field | Calculation |
|---|---|
| **Shares** | Token balance / 1e9 |
| **Avg Price** | `costBasis.totalCost / costBasis.totalShares` |
| **Current Value** | `shares * lmsrImpliedPrice` |
| **P&L** | `currentValue - (shares * avgPrice)` |
| **P&L %** | `(P&L / invested) * 100` |

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

## Resolved positions

The Resolved tab shows Won/Lost badges based on:
- **Won**: User held the winning side (YES when outcome=true, NO when outcome=false)
- **Lost**: User held the losing side

Each card links to the market page.

## Data loading

On mount, three fetches run in parallel:
1. `fetchUserPositions(wallet)` — token balances across all markets
2. `fetchEscrow(wallet)` — escrow SOL balance
3. `fetchUserTradeHistory(wallet)` — trade events for cost basis

## File references

- **Profile page**: `app/profile/page.tsx`
- **Position fetching**: `lib/mentionMarket.ts` (`fetchUserPositions`)
- **Trade history**: `lib/mentionMarket.ts` (`fetchUserTradeHistory`)
