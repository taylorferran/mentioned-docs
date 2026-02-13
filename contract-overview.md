# Contract Overview

The Mentioned program is a Solana smart contract built with Anchor. It handles custody, token minting, and payouts for binary YES/NO prediction markets on word mentions. Order matching happens off-chain via a CLOB backend — the contract only settles matched trades.

Program ID: `AJ4XSwJoh2C8vmd8U7xhpzMkzkZZPaBRpbfpkmm4DmeN`

## Instructions

| Instruction | Caller | Status | Description |
|---|---|---|---|
| [deposit](deposit.md) | User | Implemented | Deposit SOL into a per-user escrow PDA |
| [withdraw](withdraw.md) | User | Implemented | Withdraw available SOL from escrow |
| [create_market](create-market.md) | Admin | Implemented | Create a new word market with YES/NO mints and vault |
| [pause_market](pause-market.md) | Admin | Implemented | Pause an active market |
| [resolve_market](resolve-market.md) | Admin | Implemented | Resolve a market with a YES/NO outcome |
| [settle_match](settle-match.md) | Backend | Stubbed | Settle a matched trade — mint tokens, move SOL to vault |
| [claim](claim.md) | User | Stubbed | Burn winning tokens and claim SOL from vault |

## Who calls what

### User

Users interact with the contract through their wallet. They deposit SOL into their escrow before trading, and withdraw or claim after resolution.

- **deposit** — Fund their escrow account with SOL
- **withdraw** — Pull available (unlocked) SOL back to their wallet
- **claim** — After market resolution, burn winning tokens for SOL

### Admin

The admin wallet controls market lifecycle. In the MVP, this is a single trusted authority.

- **create_market** — Spin up a new word market with token mints
- **pause_market** — Halt trading on a market
- **resolve_market** — Declare the YES/NO outcome for a word

### Backend (CLOB)

The off-chain order book matches YES and NO buyers, then calls the contract to settle on-chain.

- **settle_match** — After matching two orders, the backend submits a settlement transaction that mints tokens for both parties and moves SOL into the market vault

## Account structures

### UserEscrow

Per-user account holding their deposited SOL.

Seeds: `["escrow", user_wallet]`

| Field | Type | Description |
|---|---|---|
| owner | Pubkey | User's wallet address |
| balance | u64 | Available lamports (withdrawable, usable for orders) |
| locked | u64 | Lamports committed to open orders (not withdrawable) |
| bump | u8 | PDA bump seed |

Size: 8 (discriminator) + 32 + 8 + 8 + 1 = 57 bytes

### WordMarket

Represents a single word within a market group.

Seeds: `["market", market_id (u64 LE), word_index (u16 LE)]`

| Field | Type | Description |
|---|---|---|
| authority | Pubkey | Admin wallet that can pause/resolve |
| market_id | u64 | Groups words under one market |
| word_index | u16 | Index of this word in the group |
| label | String | The word (max 32 chars) |
| yes_mint | Pubkey | SPL token mint for YES shares |
| no_mint | Pubkey | SPL token mint for NO shares |
| vault | Pubkey | SOL vault PDA address |
| total_collateral | u64 | Total lamports locked in vault |
| status | MarketStatus | Active, Paused, or Resolved |
| outcome | Option\<Outcome\> | None until resolved, then Yes or No |
| bump | u8 | PDA bump seed |
| vault_bump | u8 | Vault PDA bump seed |

### Related PDAs

Derived from the WordMarket PDA:

| Account | Seeds | Description |
|---|---|---|
| YES mint | `["yes_mint", word_market]` | SPL token mint, authority = word_market PDA |
| NO mint | `["no_mint", word_market]` | SPL token mint, authority = word_market PDA |
| Vault | `["vault", word_market]` | Holds collateral SOL |

Token decimals: 6 (1,000,000 = 1 share)

## Core invariant

**1 SOL deposited = 1 YES token + 1 NO token minted.** The vault is always solvent.

## Instruction details

### settle_match(price: u64, amount: u64)

Called by the backend after matching a YES buyer with a NO buyer.

- **price**: lamports per share the YES buyer pays (e.g. 500,000,000 = 0.5 SOL)
- **amount**: number of shares in token base units (1,000,000 = 1 share)
- NO buyer pays `(1,000,000,000 - price)` per share

Effect:
1. Deduct from `yes_buyer_escrow.locked` and `no_buyer_escrow.locked`
2. Transfer SOL from both escrows to market vault
3. Mint YES tokens to yes_buyer, NO tokens to no_buyer
4. Increment `word_market.total_collateral`

### claim()

Called by a user after market resolution.

Effect:
1. Market must be Resolved
2. Read user's balance of winning token
3. Burn winning tokens
4. Transfer equivalent SOL from vault to user wallet (1 token = 1 SOL)
5. Decrement `total_collateral`

## Backend ↔ Contract interface

The CLOB matching engine interacts with the contract via:

1. **Read escrow balances** — validate order placement (balance >= order cost)
2. **Lock funds** — when user places an order, move funds from `balance` to `locked`
3. **settle_match** — after matching, build and sign the settlement tx
4. **Read word_market status** — check which markets are tradeable

> **Still needed:**
> - `lock_funds(amount)` — backend calls to move escrow balance → locked when order placed
> - `unlock_funds(amount)` — backend calls to move locked → balance when order cancelled
