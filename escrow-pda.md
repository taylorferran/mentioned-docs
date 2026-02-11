# Escrow PDA

## What it actually is

A PDA (Program Derived Address) is a wallet address that no human owns. It's controlled entirely by your program's code. No private key exists for it — only your program can sign transactions from it.

An escrow PDA is one of these program-controlled wallets, created deterministically for each user to hold their funds.

## How it works

When a user deposits SOL, it doesn't go into one big pool. Each user gets their own escrow account, derived from their wallet address:

```
Escrow PDA address = hash(program_id + "escrow" + user_wallet)
```

The seeds are deterministic, so given any user's wallet address, anyone can compute where their escrow lives. But only your program can move SOL out of it.

### Example

```
User Alice (wallet: A1b2...):
  Escrow PDA: hash(program_id + "escrow" + A1b2...) → PDA_alice
  Balance: 5 SOL

User Bob (wallet: C3d4...):
  Escrow PDA: hash(program_id + "escrow" + C3d4...) → PDA_bob
  Balance: 12 SOL
```
