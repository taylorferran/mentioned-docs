# Fetching Markets

How the frontend reads market data directly from the Solana program.

## Overview

The app fetches `WordMarket` accounts from devnet using `getProgramAccounts` with a discriminator filter. No indexer or backend API needed — it reads raw account data and deserializes with Borsh.

## Fetching WordMarket accounts

The client library (`lib/mentionMarket.ts`) does the following:

1. Connect to devnet RPC (`https://api.devnet.solana.com`)
2. Call `getProgramAccounts` on program `AJ4XSwJoh2C8vmd8U7xhpzMkzkZZPaBRpbfpkmm4DmeN` with a `memcmp` filter matching the 8-byte WordMarket account discriminator at offset 0:
   ```
   discriminator: [19, 245, 212, 180, 55, 87, 181, 250]
   ```
3. Deserialize each account's raw bytes using Borsh layout

## Borsh deserialization layout

Each `WordMarket` account is deserialized in order:

| Field | Type | Size | Notes |
|---|---|---|---|
| authority | Pubkey | 32 bytes | Admin wallet |
| market_id | u64 LE | 8 bytes | Market group ID |
| word_index | u16 LE | 2 bytes | Word position in group |
| label | Borsh string | 4 + N bytes | u32 length prefix + UTF-8 |
| yes_mint | Pubkey | 32 bytes | YES token mint |
| no_mint | Pubkey | 32 bytes | NO token mint |
| vault | Pubkey | 32 bytes | SOL vault |
| total_collateral | u64 LE | 8 bytes | Lamports in vault |
| status | u8 | 1 byte | 0=Active, 1=Paused, 2=Resolved |
| outcome | Option\<u8\> | 1-2 bytes | 0x00=None, 0x01+0x00=Yes, 0x01+0x01=No |
| bump | u8 | 1 byte | PDA bump |
| vault_bump | u8 | 1 byte | Vault PDA bump |

## Filtering by market

The URL param determines which market to display. For a numeric ID (e.g. `/market/1`), the fetched accounts are filtered to those where `account.marketId === BigInt(urlParam)`, then sorted ascending by `wordIndex`.

## Mapping to UI

Each `WordMarket` account maps to the frontend's word row:

| WordMarket field | UI field | Notes |
|---|---|---|
| label | word | The tracked word (e.g. "Economy") |
| — | yesPrice | Hardcoded `0.50` (no price discovery yet) |
| — | noPrice | Hardcoded `0.50` |
| totalCollateral | volume | Raw lamports cast to Number |
| — | change | `0` (no historical data yet) |

Market-level metadata is derived from the group:

- **Title**: `"Market #${marketId}"`
- **Category**: `"Mentions · On-Chain"`
- **Total volume**: Sum of `totalCollateral` across all words
- **Status**: Read from the first word's account (all words in a group share lifecycle)

## Market status

| Status | Badge | Trading |
|---|---|---|
| Active | Green | Enabled |
| Paused | Yellow | Enabled (contract enforces pause separately) |
| Resolved | Grey | Disabled — button shows "Market Resolved" |

When resolved, the outcome (YES/NO) is displayed alongside the badge.

## Price discovery

Prices are currently hardcoded at 50/50. Implementing real prices requires:

- An off-chain order book where users submit limit orders
- A backend matching engine that pairs YES buyers with NO buyers
- The matched price passed to `settle_match(price, amount)` on-chain
- A `last_price` field on `WordMarket` (or off-chain API) to feed back into the UI

## Trading panel math

The UI calculates shares and potential payout from the displayed price:

```
shares          = amountInSol / activePrice
potentialPayout = shares × 1 SOL
potentialProfit = potentialPayout - amountInSol
```

Buy/sell buttons are currently UI-only — actual order submission requires the backend matching engine.

## File references

- **Page component**: `app/market/[id]/page.tsx`
- **On-chain client library**: `lib/mentionMarket.ts`
- **Program IDL**: `solana_contracts/target/idl/mention_market.json`
- **Program source**: `solana_contracts/programs/mention-market/src/`
