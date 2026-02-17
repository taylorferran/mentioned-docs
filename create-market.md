# create_market

Creates a new market with up to 8 words in a single instruction. Initializes YES and NO SPL token mints for each word via `remaining_accounts`, attaches Metaplex Token Metadata to each mint, plus a shared SOL vault.

**Caller:** Admin

## Token metadata

YES/NO token mints now have on-chain Metaplex Token Metadata, so tokens display meaningful names and symbols in wallets (Phantom, Solflare, etc.) instead of "Unknown Token".

**Naming convention:**

| Field | YES token | NO token |
|---|---|---|
| Name | `{word_label} YES` | `{word_label} NO` |
| Symbol | `{PREFIX}-Y` | `{PREFIX}-N` |
| URI | empty | empty |

`PREFIX` = first 4 alphanumeric characters of the word label, uppercased. Names are truncated to 32 chars, symbols to 10 chars.

**Examples:**

| Word | YES Name | YES Symbol | NO Name | NO Symbol |
|---|---|---|---|---|
| Bitcoin | Bitcoin YES | BITC-Y | Bitcoin NO | BITC-N |
| Economy | Economy YES | ECON-Y | Economy NO | ECON-N |

## Compute budget

This instruction requires ~800K compute units due to the metadata CPI calls (up from ~200K before metadata was added). A `ComputeBudgetProgram.setComputeUnitLimit` instruction **must** be included before the create market instruction:

```typescript
const ix = await createCreateMarketIx(/* ... */)
const computeIx = createSetComputeUnitLimitIx(800_000)
await sendIxs(signer, [computeIx, ix])
```

No other instructions (buy, sell, deposit, withdraw, etc.) require this.

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
| token_metadata_program | Program | Metaplex Token Metadata (`metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`) |
| system_program | Program | Solana system program |
| rent | Sysvar | Rent sysvar |
| remaining_accounts | | 4 accounts per word: `[yes_mint, yes_metadata, no_mint, no_metadata]` |

## PDA derivation

```
market:        ["market", market_id]
vault:         ["vault", market_id]
yes_mint:      ["yes_mint", market_id, word_index]
no_mint:       ["no_mint", market_id, word_index]
yes_metadata:  Metaplex metadata PDA derived from yes_mint
no_metadata:   Metaplex metadata PDA derived from no_mint
```

Mint authority for all YES/NO mints is the `market` PDA. Token decimals: 9.

## Logic

1. Validate label length (<= 64), word count (1-8), word label lengths (<= 32)
2. Verify `remaining_accounts` has exactly `4 * num_words` entries
3. For each word:
   - Derive and verify YES/NO mint PDAs, create mint accounts via CPI, initialize with 9 decimals
   - Create Metaplex metadata accounts for each mint with generated name/symbol
4. Initialize `MarketAccount` with all fields, status `Open`, zero collateral/fees
5. Store word states with initial quantities of 0
6. Emit `MarketCreatedEvent`

## Events

Emits `MarketCreatedEvent` on success:

| Field | Type | Description |
|---|---|---|
| market_id | u64 | Market identifier |
| label | String | Market name |
| num_words | u8 | Number of words |
| authority | Pubkey | Admin wallet |
| resolver | Pubkey | Resolution authority |
| resolves_at | i64 | Resolution deadline |
| trade_fee_bps | u16 | Fee in basis points |
| initial_b | u64 | Starting LMSR parameter |
| timestamp | i64 | Creation time |

## Errors

| Error | Condition |
|---|---|
| MarketLabelTooLong | Market label exceeds 64 chars |
| WordLabelTooLong | Any word label exceeds 32 chars |
| TooManyWords | More than 8 words |
| NoWords | Empty word list |

## Local testing

The test validator must clone the Metaplex Token Metadata program from mainnet:

```toml
# Anchor.toml
[test.validator]
url = "https://api.mainnet-beta.solana.com"

[[test.validator.clone]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
```
