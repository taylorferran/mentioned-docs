# settle_match

Settles a matched trade between a YES buyer and a NO buyer. Deducts locked SOL from both escrows, moves it to the market vault, and mints YES/NO tokens to the respective buyers.

**Caller:** Backend (CLOB)
**Status:** Stubbed — accounts defined, logic TODO

## Parameters

| Name | Type | Description |
|---|---|---|
| price | u64 | Lamports per share the YES buyer pays (e.g. 500_000_000 = 0.5 SOL) |
| amount | u64 | Number of shares in token base units (1_000_000 = 1 share) |

The NO buyer pays `(1_000_000_000 - price)` per share. Together, YES + NO cost = 1 SOL per share pair.

## Accounts

| Account | Type | Description |
|---|---|---|
| backend | Signer | Backend co-signer wallet |
| word_market | Account, mut | The market being settled |
| yes_buyer_escrow | PDA, mut | YES buyer's escrow account |
| yes_buyer | UncheckedAccount | YES buyer's wallet |
| yes_buyer_token_account | TokenAccount, mut | YES buyer's token account for YES mint |
| no_buyer_escrow | PDA, mut | NO buyer's escrow account |
| no_buyer | UncheckedAccount | NO buyer's wallet |
| no_buyer_token_account | TokenAccount, mut | NO buyer's token account for NO mint |
| yes_mint | Mint, mut | YES SPL token mint |
| no_mint | Mint, mut | NO SPL token mint |
| vault | PDA, mut | Market's SOL vault |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |

## PDA derivation

```
yes_buyer_escrow:  ["escrow", yes_buyer.key()]
no_buyer_escrow:   ["escrow", no_buyer.key()]
vault:             ["vault", word_market.key()]
```

## Intended logic

1. Calculate YES buyer cost: `price * amount / 1_000_000`
2. Calculate NO buyer cost: `(1_000_000_000 - price) * amount / 1_000_000`
3. Deduct from `yes_buyer_escrow.locked` and `no_buyer_escrow.locked`
4. Transfer SOL from both escrows to market vault
5. Mint `amount` YES tokens to `yes_buyer_token_account`
6. Mint `amount` NO tokens to `no_buyer_token_account`
7. Increment `word_market.total_collateral`

## Expected errors

| Error | Condition |
|---|---|
| InsufficientYesFunds | YES buyer's locked balance too low |
| InsufficientNoFunds | NO buyer's locked balance too low |
| InvalidPrice | Price not between 0 and 1 SOL (exclusive) |
| MathOverflow | Arithmetic overflow |

## Source

```rust
pub fn handle_settle_match(
    _ctx: Context<SettleMatch>,
    _price: u64,
    _amount: u64,
) -> Result<()> {
    // TODO: Implement settlement logic
    msg!("settle_match: not yet implemented");
    err!(MentionMarketError::NotImplemented)
}
```
