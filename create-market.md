# create_market

Creates a new market with up to 8 words in a single instruction. Initializes YES and NO SPL token mints for each word via `remaining_accounts`, plus a shared SOL vault.

**Caller:** Admin

## Parameters

| Name | Type | Description |
|---|---|---|
| market_id | u64 | Unique market identifier |
| label | String | Market name (max 64 chars) |
| word_labels | Vec\<String\> | List of words to trade on (1-8, each max 32 chars) |
| resolves_at | i64 | Unix timestamp for resolution deadline |
| resolver | Pubkey | Address authorized to resolve outcomes |
| trade_fee_bps | u16 | Fee per trade in basis points (e.g. 50 = 0.5%) |
| initial_b | u64 | Starting LMSR liquidity parameter |
| base_b_per_sol | u64 | How much 'b' scales per SOL of liquidity added |

## Accounts

| Account | Type | Description |
|---|---|---|
| authority | Signer, mut | Admin wallet (pays for account creation) |
| market | PDA, init | The new MarketAccount |
| vault | PDA | SOL vault for this market |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |
| rent | Sysvar | Rent sysvar |
| remaining_accounts | | Pairs of (yes_mint, no_mint) PDAs per word |

## PDA derivation

```
market:    ["market", market_id]
vault:     ["vault", market_id]
yes_mint:  ["yes_mint", market_id, word_index]
no_mint:   ["no_mint", market_id, word_index]
```

Mint authority for all YES/NO mints is the `market` PDA. Token decimals: 9.

## Logic

1. Validate label length (<= 64), word count (1-8), word label lengths (<= 32)
2. Verify `remaining_accounts` has exactly `2 * num_words` entries
3. For each word: derive and verify YES/NO mint PDAs, create mint accounts via CPI, initialize with 9 decimals
4. Initialize `MarketAccount` with all fields, status `Open`, zero collateral/fees
5. Store word states with initial quantities of 0

## Errors

| Error | Condition |
|---|---|
| MarketLabelTooLong | Market label exceeds 64 chars |
| WordLabelTooLong | Any word label exceeds 32 chars |
| TooManyWords | More than 8 words |
| NoWords | Empty word list |
