# withdraw_liquidity

Burn LP shares and withdraw proportional SOL from the market vault. The LMSR `b` parameter is rescaled after withdrawal.

> **LP liquidity is locked until the market resolves.** Withdrawals are only allowed after all words have been resolved. This ensures the AMM has sufficient liquidity for the full lifecycle of the market.

**Caller:** Liquidity Provider

## Parameters

| Name | Type | Description |
|---|---|---|
| shares_to_burn | u64 | Number of LP shares to redeem |

## Accounts

| Account | Type | Description |
|---|---|---|
| lp_wallet | Signer, mut | LP's wallet |
| market | Account, mut | The market |
| vault | PDA, mut | Market's SOL vault |
| lp_position | PDA, mut | LP's position account |
| system_program | Program | Solana system program |

## Logic

1. Require `market.status == MarketStatus::Resolved` (fails with `MarketNotResolved`)
2. Validate `shares_to_burn > 0` and LP has enough shares
3. Calculate SOL to return: `sol_out = shares_to_burn * vault_balance / total_lp_shares`
4. Transfer `sol_out` from vault PDA to LP wallet via CPI
5. Decrement `market.total_lp_shares`
6. Rescale LMSR `b`: `b = base_b_per_sol * new_vault_balance / 1e9`
7. Decrement LP position shares
8. Emit `LiquidityEvent`

## Errors

| Error | Condition |
|---|---|
| MarketNotResolved | Market is not in Resolved status |
| ZeroAmount | `shares_to_burn == 0` or calculated `sol_out == 0` |
| InsufficientShares | LP doesn't hold enough shares |
| EmptyPool | No LP shares exist |
| MathOverflow | Arithmetic overflow |
