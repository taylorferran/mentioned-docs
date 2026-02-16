# buy

Buy YES or NO tokens for a specific word in a market. Price is determined by the LMSR cost function. SOL is deducted from the user's escrow and transferred to the market vault.

**Caller:** User

## Parameters

| Name | Type | Description |
|---|---|---|
| word_index | u8 | Which word in the market (0-7) |
| direction | Side | `Yes` or `No` |
| quantity | u64 | Number of tokens to buy (in base units, 1e9 = 1 token) |
| max_cost | u64 | Maximum lamports willing to pay (slippage protection) |

## Accounts

| Account | Type | Description |
|---|---|---|
| trader | Signer, mut | User's wallet |
| trader_escrow | PDA, mut | User's escrow account |
| market | Account, mut | The market to trade on (must be Open) |
| vault | PDA, mut | Market's SOL vault |
| token_mint | Mint, mut | The YES or NO mint for the target word |
| trader_token_account | TokenAccount, mut | User's token account for the mint |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |

## Logic

1. Validate `quantity > 0` and `word_index` is valid
2. Verify `token_mint` matches the expected mint for the given direction
3. Calculate cost via LMSR: `C(after) - C(before)`
4. Apply trade fee: `fee = cost * trade_fee_bps / 10000`
5. Check `total_cost <= max_cost` (slippage protection)
6. Deduct `total_cost` from `trader_escrow.balance`
7. Transfer lamports from escrow PDA to vault
8. Mint `quantity` tokens to the trader's token account
9. Update word's `yes_quantity` or `no_quantity`
10. Accumulate fee on market
11. Emit `TradeEvent` with new implied price

## Trade fee

The fee is charged on top of the LMSR cost:
```
total_cost = lmsr_cost + (lmsr_cost * trade_fee_bps / 10000)
```

## Errors

| Error | Condition |
|---|---|
| ZeroAmount | `quantity == 0` |
| InvalidWordIndex | Word index out of range or mint mismatch |
| MarketNotOpen | Market is not in Open status |
| InsufficientBalance | Escrow balance too low |
| SlippageExceeded | `total_cost > max_cost` |
| MathOverflow | Arithmetic overflow |

## Events

```
TradeEvent {
  market_id, word_index, direction, quantity, cost, fee,
  new_yes_qty, new_no_qty, implied_yes_price, trader, timestamp
}
```
