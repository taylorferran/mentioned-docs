# Fetching Markets

How the frontend reads market data directly from the Solana program.

## Overview

The app fetches `MarketAccount` accounts from devnet using `getProgramAccounts` with a discriminator filter. No indexer or backend API needed — it reads raw account data and deserializes with a custom Borsh deserializer.

## Fetching MarketAccount accounts

The client library (`lib/mentionMarket.ts`) does the following:

1. Connect to devnet RPC (`https://api.devnet.solana.com`)
2. Call `getProgramAccounts` on program `2oKQaiKx3C2qpkqFYGDdvEGTyBDJP85iuQtJ5vaPdFrU` with a `memcmp` filter matching the 8-byte MarketAccount discriminator at offset 0
3. Deserialize each account's raw bytes using a custom Borsh deserializer (`deserializeMarketAccount`)

## Account structure

Each `MarketAccount` contains all data for a market, including embedded `WordState` entries (up to 8):

**MarketAccount fields:**

| Field | Type | Notes |
|---|---|---|
| version | u8 | Schema version (1) |
| bump | u8 | PDA bump |
| market_id | u64 LE | Market identifier |
| label | Borsh string | Market name (max 64 chars) |
| authority | Pubkey (32 bytes) | Admin wallet |
| resolver | Pubkey (32 bytes) | Resolution authority |
| router | Option\<Pubkey\> | V2 field |
| pool_vault | Pubkey (32 bytes) | SOL vault PDA |
| vault_bump | u8 | Vault PDA bump |
| total_lp_shares | u64 LE | LP share supply |
| liquidity_param_b | u64 LE | LMSR parameter |
| base_b_per_sol | u64 LE | B scaling rate |
| num_words | u8 | Word count (1-8) |
| words | WordState[8] | Embedded word states |
| status | u8 | 0=Open, 1=Paused, 2=Resolved |
| created_at | i64 LE | Creation timestamp |
| resolves_at | i64 LE | Resolution deadline |
| resolved_at | Option\<i64\> | Actual resolution time |
| trade_fee_bps | u16 LE | Fee in basis points |
| protocol_fee_bps | u16 LE | Protocol fee portion |
| accumulated_fees | u64 LE | Total fees collected |

**WordState fields (per word):**

| Field | Type | Notes |
|---|---|---|
| word_index | u8 | Position in market (0-7) |
| label | Borsh string | The word (max 32 chars) |
| yes_mint | Pubkey (32 bytes) | YES token mint |
| no_mint | Pubkey (32 bytes) | NO token mint |
| yes_quantity | i64 LE | Net YES tokens outstanding |
| no_quantity | i64 LE | Net NO tokens outstanding |
| outcome | Option\<bool\> | None, true (mentioned), or false |

## Fetching a single market

For the market page (`/market/[id]`), `fetchMarket(BigInt(marketId))` fetches a single market by ID — one RPC call instead of fetching all accounts.

## Mapping to UI

Each `WordState` within a `MarketAccount` maps to the frontend's word row:

| WordState field | UI field | Notes |
|---|---|---|
| label | word | The tracked word (e.g. "Economy") |
| yes_quantity / no_quantity | yesPrice / noPrice | Derived from LMSR implied price |
| — | volume | Not directly available (use vault balance) |

Market-level metadata comes from the `MarketAccount`:

- **Title**: `market.label`
- **Category**: `"Mentions · On-Chain"`
- **Event time**: derived from `market.resolvesAt`
- **Status**: `market.status` (Open / Paused / Resolved)

## Market status

| Status | Badge | Trading |
|---|---|---|
| Open | Green | Enabled |
| Paused | Yellow | Disabled |
| Resolved | Grey | Disabled — shows "Market Resolved" |

When resolved, per-word outcomes (`true`/`false`) are displayed.

## Pricing

Prices are derived from the LMSR implied price calculation:
```
p_yes = exp(q_yes / b) / (exp(q_yes / b) + exp(q_no / b))
p_no  = 1 - p_yes
```

The `yes_quantity`, `no_quantity`, and `liquidity_param_b` fields from the on-chain data feed this calculation.

## PDA seeds

| Account | Seeds |
|---|---|
| Market | `["market", market_id]` |
| Vault | `["vault", market_id]` |
| YES mint | `["yes_mint", market_id, word_index]` |
| NO mint | `["no_mint", market_id, word_index]` |

## File references

- **Page component**: `app/market/[id]/page.tsx`
- **On-chain client library**: `lib/mentionMarket.ts`
- **Program source**: `solana_contracts/programs/mention-market-amm/src/`
