# Mentioned

Mentioned is a mention market prediction platform built on Solana. Users trade on whether specific words will be mentioned in defined contexts, using an on-chain AMM with LMSR pricing.

## How it works

Markets are created around sets of words (up to 8 per market). For each word, users can buy **YES** or **NO** tokens — betting on whether that word will be mentioned. Prices are set dynamically by the LMSR (Logarithmic Market Scoring Rule) based on supply and demand. When the market resolves, winning token holders redeem their tokens for SOL.

## Key concepts

- **Market** — A collection of words to trade on, stored as a single `MarketAccount` on Solana
- **YES/NO tokens** — Minted when a user buys via the AMM, representing each side of the bet
- **LMSR** — On-chain pricing model that adjusts prices based on token quantities
- **Escrow** — Users deposit SOL before trading and withdraw after resolution
- **Liquidity** — LPs deposit SOL to deepen the market, earning trade fees
- **Resolution** — Each word is resolved individually as mentioned (true) or not (false)

## Quick links

- [MVP Flow](mvp-flow.md) — End-to-end walkthrough of the MVP
- [Contract Overview](contract-overview.md) — Full contract architecture and instructions
- [Market Types](market-types.md) — Potential market categories
