# Admin Operations

Admin page features for market management, liquidity operations, and resolution.

## Market creation

See [create_market](create-market.md) for the on-chain instruction. The admin form includes input validation:

| Field | Constraint |
|---|---|
| Market label | Max 64 characters |
| Word labels | Max 32 characters each |
| Word count | 1–8 words per market |
| Trade fee | 0–10,000 bps |

## Liquidity management

Per-market controls for adding and withdrawing liquidity.

### Add liquidity

1. Admin enters SOL amount for a specific market
2. Calls `createDepositLiquidityIx(adminWallet, marketId, amount)`
3. SOL transfers to the market vault, LP shares minted to admin's LP position account
4. Market's `liquidity_param_b` scales up: `b = base_b_per_sol * vault_balance / 1e9`

### Withdraw liquidity

1. Admin enters shares to burn
2. Calls `createWithdrawLiquidityIx(adminWallet, marketId, sharesToBurn)`
3. Proportional SOL returned from vault
4. Market's `liquidity_param_b` scales down accordingly

Each market in the admin list has its own SOL input field and Add/Withdraw buttons.

## Bulk resolve

Multi-word resolution in a single transaction.

### Flow

1. For each word in a market, toggle YES (true) or NO (false) outcome
2. Visual highlight shows selected outcome per word
3. Review summary confirmation dialog
4. `handleBulkResolve` creates a `createResolveWordIx` for each word and sends all in one transaction

This is more efficient than resolving words individually — one signature instead of N.

## Pause / Unpause

Toggle a market between Open and Paused states. See [pause_market](pause-market.md).

## File references

- **Admin page**: `app/admin/page.tsx`
- **SDK**: `lib/mentionMarket.ts`
