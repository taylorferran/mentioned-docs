# Frontend SDK

Reference for `lib/mentionMarket.ts` — the client library that wraps all on-chain interactions.

## Constants

| Export | Value | Description |
|---|---|---|
| `PROGRAM_ID` | `2oKQaiKx3C2qpkqFYGDdvEGTyBDJP85iuQtJ5vaPdFrU` | AMM program address (devnet) |
| `RENT_SYSVAR` | `SysvarRent111111111111111111111111111111111` | Sysvar rent address |
| `ASSOCIATED_TOKEN_PROGRAM` | `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | SPL Associated Token Program |
| `createRpc()` | — | Factory that creates a Solana devnet RPC client |

## PDA helpers

All PDAs are derived from the program ID using these seed patterns:

| Helper | Seeds | Returns |
|---|---|---|
| `getMarketPDA(marketId)` | `["market", marketId]` | Market account address |
| `getVaultPDA(marketId)` | `["vault", marketId]` | SOL vault address |
| `getYesMintPDA(marketId, wordIndex)` | `["yes_mint", marketId, wordIndex]` | YES token mint |
| `getNoMintPDA(marketId, wordIndex)` | `["no_mint", marketId, wordIndex]` | NO token mint |
| `getEscrowPDA(wallet)` | `["escrow", wallet]` | User escrow account |
| `getLpPositionPDA(marketId, lpWallet)` | `["lp", marketId, lpWallet]` | LP position account |
| `getAssociatedTokenAddress(mint, owner)` | Standard ATA derivation | User's token account for a given mint |

## Instruction builders

Each function returns a transaction instruction ready to be signed and sent.

| Export | Parameters | Description |
|---|---|---|
| `createDepositIx` | `(user, amount)` | Deposit SOL into user escrow |
| `createWithdrawIx` | `(user, amount)` | Withdraw SOL from user escrow |
| `createCreateMarketIx` | `(admin, marketId, params)` | Create a new market with word mints |
| `createPauseMarketIx` | `(admin, marketId)` | Toggle market Open/Paused |
| `createBuyIx` | `(trader, marketId, wordIndex, side, quantity, maxCost, market)` | Buy YES/NO tokens via LMSR |
| `createSellIx` | `(trader, marketId, wordIndex, side, quantity, minReturn, market)` | Sell YES/NO tokens back to AMM |
| `createRedeemIx` | `(trader, marketId, wordIndex, side, market)` | Burn winning tokens after resolution |
| `createDepositLiquidityIx` | `(lpWallet, marketId, amount)` | LP deposits SOL into AMM pool |
| `createWithdrawLiquidityIx` | `(lpWallet, marketId, sharesToBurn)` | LP withdraws SOL proportional to shares |
| `createResolveWordIx` | `(resolver, marketId, wordIndex, outcome)` | Resolve a single word as true/false |
| `createAtaIx` | `(payer, owner, mint)` | Create an Associated Token Account |

### Buy instruction detail

`createBuyIx` requires the full `market` object because it needs to derive the correct YES/NO mint PDAs and the user's ATA addresses. The `maxCost` parameter provides slippage protection — the transaction reverts if the LMSR cost exceeds this value.

### Sell instruction detail

`createSellIx` similarly needs the `market` object for mint/ATA derivation. The `minReturn` parameter is the slippage floor — the transaction reverts if the LMSR return is below this value.

### ATA creation

`createAtaIx` is called before a user's first buy for a given mint. The trading flow checks whether the user's ATA exists and prepends this instruction if needed.

## Client-side LMSR math

Full JavaScript reimplementation of the on-chain LMSR pricing for UI cost/return previews.

### Core functions

| Function | Signature | Description |
|---|---|---|
| `lmsrCostFn` | `(qYes, qNo, b) → number` | Raw cost function: `C = b * ln(e^(qY/b) + e^(qN/b))` |
| `logSumExp` | `(a, b) → number` | Numerically stable log-sum-exp helper |
| `lmsrImpliedPrice` | `(yesQty, noQty, b) → { yes, no }` | Computes YES/NO prices using softmax |
| `lmsrBuyCost` | `(yesQty, noQty, side, amount, b) → number` | Cost to buy `amount` shares |
| `lmsrSellReturn` | `(yesQty, noQty, side, amount, b) → number` | Return from selling `amount` shares |

### Pricing formula

```
p_yes = exp(q_yes / b) / (exp(q_yes / b) + exp(q_no / b))
p_no  = 1 - p_yes
```

### Buy cost

```
buyCost = C(q_yes + amount, q_no, b) - C(q_yes, q_no, b)    // for YES side
buyCost = C(q_yes, q_no + amount, b) - C(q_yes, q_no, b)    // for NO side
```

### Sell return

```
sellReturn = C(q_yes, q_no, b) - C(q_yes - amount, q_no, b)    // for YES side
sellReturn = C(q_yes, q_no, b) - C(q_yes, q_no - amount, b)    // for NO side
```

### Precision note

The client-side math uses JavaScript floating-point (`Math.exp` / `Math.log`), while the on-chain math uses fixed-point `i128` at 1e9 precision. There may be minor precision drift between what the UI previews and what the contract charges. The slippage buffer on `maxCost` / `minReturn` accounts for this.

## File reference

- **Source**: `lib/mentionMarket.ts`
- **Used by**: `app/market/[id]/page.tsx`, `app/profile/page.tsx`, `app/admin/page.tsx`, `components/Header.tsx`
