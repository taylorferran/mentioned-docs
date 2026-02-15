# Fetching User Positions

How we fetch and display real SPL token positions across the frontend.

## Overview

When a trade is matched via `settle_match`, the program mints YES and NO SPL tokens to the respective buyers. The frontend reads these token balances directly from Solana and cross-references them against known WordMarket mint addresses to determine what positions a user holds.

## Data flow

```
User's Wallet
    │
    ▼
getTokenAccountsByOwner(wallet, { programId: TOKEN_PROGRAM })
    │
    │  Returns all SPL token accounts owned by the wallet
    ▼
fetchAllWordMarkets()
    │
    │  Returns all WordMarket accounts (getProgramAccounts + Borsh decode)
    ▼
Cross-reference: token account mint ↔ WordMarket.yesMint / WordMarket.noMint
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

1. **Parallel fetch** — calls `fetchAllWordMarkets()` and `rpc.getTokenAccountsByOwner()` simultaneously
2. **Build lookup maps** — creates `yesMintMap` and `noMintMap` keyed by mint address, pointing to the WordMarket account
3. **Match tokens** — iterates through the user's token accounts, checks if each mint exists in either map
4. **Build positions** — for each match, calculates shares (raw amount / 10^6) and estimated value

### RPC call

```typescript
rpc.getTokenAccountsByOwner(
  userAddr,
  { programId: TOKEN_PROGRAM },
  { encoding: 'jsonParsed' }
).send()
```

Returns parsed token account data:
```json
{
  "parsed": {
    "info": {
      "mint": "<mint address>",
      "owner": "<wallet address>",
      "tokenAmount": {
        "amount": "1000000",
        "decimals": 6,
        "uiAmount": 1.0
      }
    }
  }
}
```

### Token math

- Token decimals: **6** (set during `create_market`)
- 1 share = 1,000,000 base units
- `shares = Number(rawAmount) / 1_000_000`

### Position valuation

No price discovery exists yet, so estimated values use placeholders:

| Market Status | Side | Value per Share |
|---|---|---|
| Active | YES/NO | 0.50 SOL |
| Paused | YES/NO | 0.50 SOL |
| Resolved | Winner | 1.00 SOL |
| Resolved | Loser | 0.00 SOL |

This will be replaced with real pricing once an order book or AMM is implemented.

### Claimable logic

A position is claimable when all three conditions are true:

1. `market.status === MarketStatus.Resolved`
2. User holds the winning side (`Outcome.Yes` → YES tokens, `Outcome.No` → NO tokens)
3. `rawAmount > 0`

## UserPosition type

```typescript
export interface UserPosition {
  wordMarketPubkey: Address
  market: WordMarket
  side: 'YES' | 'NO'
  rawAmount: bigint
  shares: number
  estimatedValueSol: number
  claimable: boolean
}
```

## Where positions are displayed

### Header (`components/Header.tsx`)

Portfolio value includes position value, not just escrow balance. Polls every 15 seconds.

```
Portfolio: 1.63 SOL    ←  escrow (1.13) + positions (0.50)
Cash:      1.13 SOL    ←  escrow only
```

### Profile page (`app/profile/page.tsx`)

Fetches positions and escrow on mount:

```typescript
Promise.all([
  fetchUserPositions(toAddress(publicKey)),
  fetchEscrow(toAddress(publicKey)),
])
```

**Portfolio cards:**
- Escrow Balance — from `fetchEscrow()`
- Active Positions — sum of `estimatedValueSol` for Active/Paused markets
- Claimable — sum of `estimatedValueSol` for claimable positions
- Total Positions — count of all positions

**Tabs:**
- **Active** — positions where `market.status` is Active or Paused
- **Claimable** — positions where `claimable === true`
- **History** — placeholder (no on-chain event log yet)

### Market page (`app/market/[id]/page.tsx`)

Shows "Your Position" section in the trading panel for the selected word:

```typescript
const selectedWordPosition = useMemo(() => {
  if (!selectedWord) return null
  return userPositions.find((p) => p.market.label === selectedWord) ?? null
}, [selectedWord, userPositions])
```

Displays:
```
Your Position
Side:       YES
Shares:     1.00
Est. Value: 0.5000 SOL
```

Only visible when the user has tokens for the currently selected word.

## Dependencies

All position fetching uses `@solana/kit` v6 (not `@solana/web3.js`):

- `createSolanaRpc(devnet(DEVNET_URL))` — creates RPC client
- `getTokenAccountsByOwner` — fetches user's SPL token accounts
- `getProgramAccounts` — fetches all WordMarket accounts

## Not yet implemented

- **Claim transaction** — button exists but is a `console.log` TODO. The claim discriminator is defined.
- **Real pricing** — all active positions valued at 0.50 SOL/share. Needs order book.
- **P&L / cost basis** — no purchase price stored on-chain. Would need trade history indexing.
- **History tab** — no on-chain event log for settled/claimed positions yet.
- **Withdraw UI** — `withdraw` instruction exists in the contract but no UI button.
