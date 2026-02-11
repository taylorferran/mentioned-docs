# Contract Overview

The Mentioned program is a Solana smart contract built with Anchor. It handles custody, token minting, and payouts for binary YES/NO prediction markets on word mentions. Order matching happens off-chain via a CLOB backend — the contract only settles matched trades.

## Instructions

| Instruction | Caller | Description |
|---|---|---|
| [deposit](deposit.md) | User | Deposit SOL into a per-user escrow PDA |
| [withdraw](withdraw.md) | User | Withdraw available SOL from escrow |
| [create_market](create-market.md) | Admin | Create a new word market with YES/NO mints and vault |
| [pause_market](pause-market.md) | Admin | Pause an active market |
| [resolve_market](resolve-market.md) | Admin | Resolve a market with a YES/NO outcome |
| [settle_match](settle-match.md) | Backend | Settle a matched trade — mint tokens, move SOL to vault |
| [claim](claim.md) | User | Burn winning tokens and claim SOL from vault |

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

```
seeds: ["escrow", user_wallet]
```

| Field | Type | Description |
|---|---|---|
| owner | Pubkey | User's wallet address |
| balance | u64 | Available lamports (withdrawable, usable for orders) |
| locked | u64 | Lamports committed to open orders (not withdrawable) |
| bump | u8 | PDA bump seed |

### WordMarket

Represents a single word within a market group.

```
seeds: ["market", market_id (u64 LE), word_index (u16 LE)]
```

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

## Core invariant

**1 SOL deposited = 1 YES token + 1 NO token minted.** The vault is always solvent.
