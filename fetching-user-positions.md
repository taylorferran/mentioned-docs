# Fetching User Positions

How we fetch and display real SPL token positions across the frontend.

## Overview

When a user buys tokens via the `buy` instruction, the AMM mints YES or NO SPL tokens. The frontend reads these token balances directly from Solana and cross-references them against known MarketAccount mint addresses to determine what positions a user holds.

## Data flow

```
User's Wallet
    │
    ▼
getTokenAccountsByOwner(wallet, { programId: TOKEN_PROGRAM })
    │
    │  Returns all SPL token accounts owned by the wallet
    ▼
fetchAllMarkets()
    │
    │  Returns all MarketAccount accounts (getProgramAccounts + Borsh decode)
    ▼
Cross-reference: token account mint ↔ WordState.yesMint / WordState.noMint
    │
    ▼
UserPosition[] — side, shares, estimated value, claimable status
```

## Core function: `fetchUserPositions()`

**File:** `lib/mentionMarket.ts`

```typescript
export async function fetchUserPositions(
  userAddr: Address
): Promise<UserPosition[]>
```

### How it works

1. **Parallel fetch** — calls `fetchAllMarkets()` and `rpc.getTokenAccountsByOwner()` simultaneously
2. **Build lookup maps** — creates mint-to-market maps keyed by mint address, pointing to the MarketAccount and word info
3. **Match tokens** — iterates through the user's token accounts, checks if each mint exists in the maps. For resolved markets, 0-balance token accounts are included (so redeemed/sold positions still appear)
4. **Build positions** — for each match, calculates shares (raw amount / 1e9), estimated value, and win/loss status

### Token math

- Token decimals: **9** (set during `create_market`)
- 1 share = 1,000,000,000 base units
- `shares = Number(rawAmount) / 1_000_000_000`

### Position valuation

Estimated values use LMSR implied prices for active markets:

| Market Status | Side | Value per Share |
|---|---|---|
| Open | YES/NO | LMSR implied price (or 0.50 fallback) |
| Paused | YES/NO | Last implied price (or 0.50 fallback) |
| Resolved | Winner | 1.00 SOL |
| Resolved | Loser | 0.00 SOL |

### Claimable logic

A position is claimable when all three conditions are true:

1. The word's `outcome` is set (not None)
2. User holds the winning side (`outcome == true` → YES tokens, `outcome == false` → NO tokens)
3. `rawAmount > 0`

## UserPosition type

```typescript
export interface UserPosition {
  marketId: bigint
  wordIndex: number
  wordLabel: string
  marketLabel: string
  marketStatus: MarketStatus
  side: 'YES' | 'NO'
  rawAmount: bigint
  shares: number
  estimatedValueSol: number
  claimable: boolean
  won: boolean | null    // true = winning side, false = losing side, null = unresolved
}
```

### Resolved position states

The `won` field combined with token balance determines the display state:

| Condition | Label | Meaning |
|---|---|---|
| `won && balance === 0` | **Claimed** | User already redeemed winnings |
| `won && balance > 0` | **Won** | Claimable — user can still redeem |
| `!won && balance === 0` | **Sold** | User sold before resolution |
| `!won && balance > 0` | **Lost** | Tokens are worthless |

## Where positions are displayed

### Header (`components/Header.tsx`)

Portfolio value includes position value plus escrow balance. Polls every 15 seconds.

```
Portfolio: 1.63 SOL    ←  escrow (1.13) + positions (0.50)
Cash:      1.13 SOL    ←  escrow only
```

### Profile page (`app/profile/page.tsx`)

Fetches positions and escrow on mount. See [Portfolio & Cost Basis](portfolio.md) for full details on the profile dashboard, including the four-tab interface (Active, Claimable, Resolved, History) and P&L tracking.

### Market page (`app/market/[id]/page.tsx`)

Shows "Your Position" for the selected word in the trading panel. Filters positions by `marketId` and matches by `wordLabel`.

## Dependencies

All position fetching uses `@solana/kit` v6:

- `createSolanaRpc(devnet(DEVNET_URL))` — creates RPC client
- `getTokenAccountsByOwner` — fetches user's SPL token accounts
- `getProgramAccounts` — fetches all MarketAccount accounts

## Related pages

- [Portfolio & Cost Basis](portfolio.md) — P&L tracking and cost basis computation from trade history
- [Trade History](trade-history.md) — How trade events are parsed from on-chain logs
- [Scalability Roadmap](scalability.md) — Planned migration to an indexer for position fetching
