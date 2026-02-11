# claim

Burns the user's winning tokens and transfers the equivalent SOL from the market vault to their wallet. 1 winning token = 1 SOL.

**Caller:** User
**Status:** Stubbed — accounts defined, logic TODO

## Parameters

None.

## Accounts

| Account | Type | Description |
|---|---|---|
| user | Signer, mut | User's wallet (receives SOL payout) |
| word_market | Account, mut | The resolved market |
| user_yes_account | TokenAccount, mut | User's YES token account |
| user_no_account | TokenAccount, mut | User's NO token account |
| yes_mint | Mint, mut | YES SPL token mint |
| no_mint | Mint, mut | NO SPL token mint |
| vault | PDA, mut | Market's SOL vault |
| token_program | Program | SPL Token program |
| system_program | Program | Solana system program |

## PDA derivation

```
vault: ["vault", word_market.key()]
```

## Intended logic

1. Validate market status is `Resolved`
2. Read the outcome (`Yes` or `No`)
3. Check user's balance of the winning token
4. Burn the winning tokens from the user's account
5. Transfer equivalent SOL from vault to user wallet (1 token = 1 SOL)
6. Decrement `word_market.total_collateral`

## Expected errors

| Error | Condition |
|---|---|
| MarketNotResolved | Market has not been resolved yet |
| NothingToClaim | User holds no winning tokens |

## Source

```rust
pub fn handle_claim(_ctx: Context<Claim>) -> Result<()> {
    // TODO: Implement claim logic
    msg!("claim: not yet implemented");
    err!(MentionMarketError::NotImplemented)
}
```
