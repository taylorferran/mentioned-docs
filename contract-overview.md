# Contract Overview

The Mentioned AMM is a Solana smart contract built with Anchor. It handles custody, LMSR-based pricing, token minting, liquidity provision, and payouts for binary YES/NO prediction markets on word mentions.

Program ID (AMM): `2oKQaiKx3C2qpkqFYGDdvEGTyBDJP85iuQtJ5vaPdFrU` (devnet)
Program ID (legacy CLOB): `AJ4XSwJoh2C8vmd8U7xhpzMkzkZZPaBRpbfpkmm4DmeN` (devnet)

## Architecture

A single `MarketAccount` PDA per market contains up to 8 embedded `WordState` entries. All words share one vault, one liquidity pool, and one status lifecycle. Pricing is handled on-chain via LMSR — no off-chain order book required for trading.

## Instructions

| Instruction | Caller | Event | Description |
|---|---|---|---|
| [deposit](deposit.md) | User | EscrowEvent | Deposit SOL into a per-user escrow PDA |
| [withdraw](withdraw.md) | User | EscrowEvent | Withdraw available SOL from escrow |
| [create_market](create-market.md) | Admin | MarketCreatedEvent | Create a market with N words, mints, metadata, and vault |
| [pause_market](pause-market.md) | Admin | MarketPausedEvent | Toggle pause/unpause on a market |
| [deposit_liquidity](deposit-liquidity.md) | LP | LiquidityEvent | Deposit SOL into the AMM pool, receive LP shares |
| [withdraw_liquidity](withdraw-liquidity.md) | LP | LiquidityEvent | Withdraw SOL proportional to LP shares |
| [buy](buy.md) | User | TradeEvent | Buy YES/NO tokens for a word using LMSR pricing |
| [sell](sell.md) | User | TradeEvent | Sell YES/NO tokens back to the AMM |
| [resolve_word](resolve-word.md) | Resolver | ResolutionEvent | Resolve a single word as mentioned (true) or not (false) |
| [redeem](redeem.md) | User | RedemptionEvent | Burn winning tokens and receive SOL from vault |

All instructions emit Anchor events for indexer support. See [Indexer Plan](INDEXER_PLAN.md) for how these events are captured.

## Who calls what

### User

Users interact with the contract through their wallet. They deposit SOL into their escrow, buy/sell tokens on word markets, and redeem winnings after resolution.

- **deposit** — Fund their escrow account with SOL
- **withdraw** — Pull available SOL back to their wallet
- **buy** — Purchase YES or NO tokens for a word (LMSR pricing)
- **sell** — Sell tokens back to the AMM
- **redeem** — After word resolution, burn winning tokens for SOL

### Admin

The admin wallet controls market creation and can pause/unpause markets.

- **create_market** — Create a new market with up to 8 words
- **pause_market** — Toggle pause on a market (halts trading)

### Resolver

A designated resolver address (set at market creation) handles outcome determination.

- **resolve_word** — Declare whether a word was mentioned (true/false). Market becomes `Resolved` when all words are resolved.

### Liquidity Provider (LP)

LPs provide SOL to the AMM pool, deepening liquidity and reducing price impact.

- **deposit_liquidity** — Deposit SOL, receive LP shares
- **withdraw_liquidity** — Burn LP shares, receive proportional SOL (only after market resolves)

## Account structures

### MarketAccount

One per market. Contains all word states, pool config, and lifecycle data.

Seeds: `["market", market_id (u64 LE)]`

| Field | Type | Description |
|---|---|---|
| version | u8 | Schema version (currently 1) |
| bump | u8 | PDA bump seed |
| market_id | u64 | Numeric ID (used in PDAs and frontend URLs) |
| label | String | Market name (max 64 chars) |
| authority | Pubkey | Admin wallet |
| resolver | Pubkey | Address authorized to resolve outcomes |
| router | Option\<Pubkey\> | V2: authorized router program |
| pool_vault | Pubkey | SOL vault PDA address |
| vault_bump | u8 | Vault PDA bump |
| total_lp_shares | u64 | Outstanding LP share tokens |
| liquidity_param_b | u64 | LMSR 'b' parameter (scaled 1e9) |
| base_b_per_sol | u64 | How much 'b' scales per SOL of liquidity |
| num_words | u8 | Number of words (max 8) |
| words | WordState[8] | Embedded word states |
| status | MarketStatus | Open, Paused, or Resolved |
| created_at | i64 | Unix timestamp of creation |
| resolves_at | i64 | Scheduled resolution deadline |
| resolved_at | Option\<i64\> | Actual resolution timestamp |
| trade_fee_bps | u16 | Fee per trade in basis points |
| protocol_fee_bps | u16 | Protocol's portion of trade fee |
| accumulated_fees | u64 | Total fees collected |
| _reserved | [u8; 256] | V2 extension space |

### WordState (embedded in MarketAccount)

| Field | Type | Description |
|---|---|---|
| word_index | u8 | Index within the market (0-7) |
| label | String | The word (max 32 chars) |
| yes_mint | Pubkey | SPL token mint for YES tokens |
| no_mint | Pubkey | SPL token mint for NO tokens |
| yes_quantity | i64 | Net YES tokens outstanding (scaled 1e9) |
| no_quantity | i64 | Net NO tokens outstanding (scaled 1e9) |
| outcome | Option\<bool\> | None = unresolved, true = mentioned, false = not |
| _reserved | [u8; 32] | Per-word extension space |

### UserEscrow

Per-user account holding their deposited SOL. Unchanged from v1.

Seeds: `["escrow", user_wallet]`

| Field | Type | Description |
|---|---|---|
| owner | Pubkey | User's wallet address |
| balance | u64 | Available lamports |
| locked | u64 | Reserved for future CLOB (currently unused) |
| bump | u8 | PDA bump seed |

### LpPosition

Per-LP-per-market account tracking LP shares.

Seeds: `["lp", market_id (u64 LE), lp_wallet]`

| Field | Type | Description |
|---|---|---|
| version | u8 | Schema version |
| bump | u8 | PDA bump seed |
| market | Pubkey | Parent market account |
| owner | Pubkey | LP's wallet |
| shares | u64 | Number of LP shares held |
| deposited_at | i64 | Timestamp of last deposit |

### PDA seeds

| Account | Seeds |
|---|---|
| Market | `["market", market_id]` |
| Vault | `["vault", market_id]` |
| YES mint | `["yes_mint", market_id, word_index]` |
| NO mint | `["no_mint", market_id, word_index]` |
| Escrow | `["escrow", user_wallet]` |
| LP Position | `["lp", market_id, lp_wallet]` |

Token decimals: 9 (1,000,000,000 = 1 share = 1 SOL payout on winning redemption)

## Token metadata

YES/NO token mints have on-chain Metaplex Token Metadata so they display correctly in wallets (Phantom, Solflare, etc.). Names follow `"{word} YES"` / `"{word} NO"` and symbols use a 4-char prefix like `BITC-Y` / `BITC-N`. See [create_market](create-market.md) for details.

## LMSR pricing

The contract uses a Logarithmic Market Scoring Rule for on-chain price discovery.

**Cost function:**
```
C(q_yes, q_no) = b * ln(exp(q_yes / b) + exp(q_no / b))
```

- **b** = liquidity parameter (higher = less price impact per trade)
- **q_yes / q_no** = net token quantities outstanding
- Buy cost = `C(after) - C(before)`
- Sell return = `C(before) - C(after)`

**Implied YES price:**
```
p_yes = exp(q_yes / b) / (exp(q_yes / b) + exp(q_no / b))
```

All math uses fixed-point arithmetic at 1e9 precision with Taylor series approximations for exp/ln.

## Core invariant

**1 winning token = 1 SOL.** After resolution, holders of the winning side's tokens redeem them 1:1 for SOL from the vault.
