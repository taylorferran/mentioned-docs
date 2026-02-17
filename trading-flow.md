# Trading Flow

End-to-end flow for buying, selling, and claiming tokens in the trading UI.

## Buy flow

```
User selects word → chooses YES/NO → enters quantity
    │
    ▼
lmsrBuyCost(yesQty, noQty, side, quantity, b)
    │  → computes cost in SOL
    │  → adds trade fee (trade_fee_bps)
    │  → applies 10% slippage buffer for maxCost
    ▼
Check ATA exists for the target mint
    │  → if not, prepend createAtaIx to transaction
    ▼
createBuyIx(trader, marketId, wordIndex, side, quantity, maxCost, market)
    │
    ▼
Sign transaction via wallet adapter (signer from useWallet())
    │
    ▼
Send + confirm transaction
    │
    ▼
Refresh market data, positions, and trade history
```

### Trade states

The UI tracks a multi-phase trading state for feedback:

| State | Display |
|---|---|
| `signing` | Waiting for wallet signature |
| `confirming` | Transaction sent, waiting for confirmation |
| `refreshing` | Confirmed, refreshing on-chain data |
| *(success)* | Green success message with auto-dismiss timer |
| *(error)* | Red error message |

### ATA handling

Before the first buy of a specific token (e.g. YES tokens for word 0), the user may not have an Associated Token Account for that mint. The buy flow:

1. Derives the ATA address via `getAssociatedTokenAddress(mint, owner)`
2. Checks if the account exists on-chain
3. If missing, prepends `createAtaIx(payer, owner, mint)` to the transaction

This is transparent to the user — the ATA creation and buy happen in a single transaction.

### Slippage protection

LMSR prices shift as other users trade. The `maxCost` parameter on buy instructions prevents overpaying:

```
maxCost = computedCost * 1.10    // 10% buffer
```

If the actual LMSR cost exceeds `maxCost` at execution time, the transaction reverts.

## Sell flow

```
User selects word → chooses YES/NO → enters quantity to sell
    │
    ▼
lmsrSellReturn(yesQty, noQty, side, quantity, b)
    │  → computes return in SOL
    ▼
createSellIx(trader, marketId, wordIndex, side, quantity, minReturn, market)
    │  → minReturn = computedReturn * 0.90 (10% slippage floor)
    ▼
Sign → Send → Confirm → Refresh
```

The sell flow burns the user's tokens and transfers SOL from the vault to the user's escrow via CPI.

## Claim / Redeem flow

After a market resolves, users holding winning tokens can redeem them for SOL.

```
Market resolved → positions checked for claimable status
    │
    ▼
For each claimable position:
    createRedeemIx(trader, marketId, wordIndex, side, market)
    │
    ▼
Batch all redeem instructions into a single transaction
    │
    ▼
Sign → Send → Confirm → Refresh
```

A position is claimable when:
1. The word's `outcome` is set (not None)
2. User holds the winning side (outcome `true` → YES tokens, outcome `false` → NO tokens)
3. `rawAmount > 0`

Redemption pays 1 token = 1 SOL, deposited into the user's escrow.

## Denomination switching

The trading panel supports two input modes:

| Mode | Input | Conversion |
|---|---|---|
| **Shares** | Direct share count | Used as-is for quantity |
| **USD** | Dollar amount | `shares = (amountUSD / SOL_USD_RATE) / activePrice` |

A dropdown in the trade panel toggles between modes.

## Live pricing

The trading panel shows real-time cost/return previews as the user adjusts quantity:

- **Buy**: estimated cost, potential payout (quantity at $1), and profit
- **Sell**: estimated return based on current LMSR state

Prices are computed from the on-chain `yesQuantity`, `noQuantity`, and `liquidityParamB` via `lmsrImpliedPrice`.

## Auto-polling

The market page refreshes data every 15 seconds via `setInterval`:

1. Fetch updated `MarketAccount` state
2. Fetch latest trade history
3. Fetch user positions (if wallet connected)

Updated values flash green briefly using the `FlashValue` component.

**Resolved markets**: Auto-polling stops entirely for resolved markets — no unnecessary RPC calls. Chart price lines freeze at the last trade point rather than extending to "now".

## Mobile trading

On mobile viewports, the trading panel renders as a bottom sheet:

- Slide-up animation with backdrop overlay
- Same functionality as the desktop sidebar panel
- Triggered by tapping the word row or a trade button

## File references

- **Market page**: `app/market/[id]/page.tsx`
- **SDK**: `lib/mentionMarket.ts`
- **FlashValue component**: `components/FlashValue.tsx`
