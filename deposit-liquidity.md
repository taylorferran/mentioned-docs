# deposit_liquidity

Deposit SOL into the AMM liquidity pool and receive LP shares. Liquidity deepens the market — higher pool balance means less price impact per trade (the LMSR `b` parameter scales with vault balance).

**Caller:** Liquidity Provider

## Parameters

| Name | Type | Description |
|---|---|---|
| amount | u64 | Lamports to deposit |

## Accounts

| Account | Type | Description |
|---|---|---|
| lp_wallet | Signer, mut | LP's wallet |
| market | Account, mut | The market (must be Open) |
| vault | PDA, mut | Market's SOL vault |
| lp_position | PDA, init_if_needed | LP's position account (created on first deposit) |
| system_program | Program | Solana system program |

## PDA derivation

```
lp_position: ["lp", market_id, lp_wallet]
```

## Logic

1. Validate `amount > 0`
2. Transfer SOL from LP wallet to vault
3. Calculate LP shares:
   - First deposit: `shares = amount` (1:1)
   - Subsequent: `shares = amount * total_lp_shares / vault_balance_before`
4. Increment `market.total_lp_shares`
5. Rescale LMSR `b`: `b = base_b_per_sol * vault_balance / 1e9`
6. Update LP position with new shares
7. Emit `LiquidityEvent`

## Dynamic b scaling

The liquidity parameter `b` scales proportionally to the vault balance:
```
b = base_b_per_sol * vault_balance / 1e9
```

More liquidity → higher `b` → less price impact per trade → tighter spreads.

## Errors

| Error | Condition |
|---|---|
| ZeroAmount | `amount == 0` |
| MarketNotOpen | Market is not in Open status |
| MathOverflow | Arithmetic overflow |
