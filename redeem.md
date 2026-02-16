# redeem

Burn winning tokens for a resolved word and receive SOL from the market vault. 1 token (1e9 base units) = 1 SOL. Payout goes to the user's escrow account.

**Caller:** User

## Parameters

| Name | Type | Description |
|---|---|---|
| word_index | u8 | Which word to redeem for (0-7) |
| direction | Side | `Yes` or `No` (must match winning side) |

## Accounts

| Account | Type | Description |
|---|---|---|
| trader | Signer, mut | User's wallet |
| trader_escrow | PDA, mut | User's escrow (receives SOL payout) |
| market | Account, mut | The market |
| vault | PDA, mut | Market's SOL vault |
| token_mint | Mint, mut | The winning token's mint |
| trader_token_account | TokenAccount, mut | User's token account holding winning tokens |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |

## Logic

1. Validate `word_index` is valid
2. Validate word is resolved (`outcome` is not None)
3. Validate `direction` matches the winning side:
   - `outcome == true` → YES wins
   - `outcome == false` → NO wins
4. Verify mint matches the direction
5. Get user's token balance (must be > 0)
6. Payout = token amount in lamports (1:1, since tokens have 9 decimals matching lamport precision)
7. Burn all winning tokens from user's account
8. Transfer payout from vault PDA to escrow PDA via CPI
9. Credit payout to `trader_escrow.balance`
10. Emit `RedemptionEvent`

## Payout math

Tokens use 9 decimals to match SOL's lamport precision:
```
1 token = 1,000,000,000 base units = 1 SOL = 1,000,000,000 lamports
payout = token_amount (base units) = token_amount (lamports)
```

## Errors

| Error | Condition |
|---|---|
| InvalidWordIndex | Word index out of range |
| WordNotResolved | Word has no outcome yet |
| NotWinningDirection | Direction doesn't match the winning side |
| NothingToRedeem | User holds 0 tokens |

## Events

```
RedemptionEvent {
  market_id, word_index, direction, tokens_burned, sol_paid, redeemer, timestamp
}
```
