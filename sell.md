# sell

Sell YES or NO tokens back to the AMM for a specific word. Return is determined by the LMSR cost function. Tokens are burned, SOL is transferred from the vault to the user's escrow.

**Caller:** User

## Parameters

| Name | Type | Description |
|---|---|---|
| word_index | u8 | Which word in the market (0-7) |
| direction | Side | `Yes` or `No` |
| quantity | u64 | Number of tokens to sell (in base units, 1e9 = 1 token) |
| min_return | u64 | Minimum lamports to accept (slippage protection) |

## Accounts

| Account | Type | Description |
|---|---|---|
| trader | Signer, mut | User's wallet |
| trader_escrow | PDA, mut | User's escrow account |
| market | Account, mut | The market (must be Open) |
| vault | PDA, mut | Market's SOL vault |
| token_mint | Mint, mut | The YES or NO mint for the target word |
| trader_token_account | TokenAccount, mut | User's token account for the mint |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |

## Logic

1. Validate `quantity > 0`, `word_index` is valid, mint matches direction
2. Verify trader has enough tokens
3. Calculate return via LMSR: `C(before) - C(after)`
4. Apply trade fee: `fee = gross_return * trade_fee_bps / 10000`
5. Check `net_return >= min_return` (slippage protection)
6. Burn `quantity` tokens from trader's account
7. Transfer `net_return` lamports from vault PDA to escrow PDA via CPI
8. Credit `net_return` to `trader_escrow.balance`
9. Update word's `yes_quantity` or `no_quantity`
10. Accumulate fee on market
11. Emit `TradeEvent` with new implied price

## Errors

| Error | Condition |
|---|---|
| ZeroAmount | `quantity == 0` |
| InvalidWordIndex | Word index out of range or mint mismatch |
| MarketNotOpen | Market is not in Open status |
| InsufficientTokens | Trader doesn't hold enough tokens |
| SlippageBelowMin | `net_return < min_return` |
| MathOverflow | Arithmetic overflow |
