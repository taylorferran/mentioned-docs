# withdraw

Withdraws available SOL from the user's escrow PDA back to their wallet.

**Caller:** User
**Status:** Implemented

## Parameters

| Name | Type | Description |
|---|---|---|
| amount | u64 | Lamports to withdraw (must be > 0) |

## Accounts

| Account | Type | Description |
|---|---|---|
| user | Signer, mut | User's wallet |
| escrow | PDA, mut | User's escrow account |
| system_program | Program | Solana system program |

## PDA derivation

```
seeds: ["escrow", user.key()]
```

## Logic

1. Validate `amount > 0`
2. Check `amount <= escrow.balance`
3. Decrement `escrow.balance` by `amount` (checked sub)
4. Transfer lamports directly from escrow PDA to user wallet
5. Emit `EscrowEvent` with action `Withdraw`

## Errors

| Error | Condition |
|---|---|
| ZeroAmount | `amount == 0` |
| InsufficientBalance | `amount > escrow.balance` |
| NotOwner | Signer doesn't match `escrow.owner` |
| MathOverflow | Balance subtraction overflows |

## Source

```rust
pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, AmmError::ZeroAmount);

    let escrow = &mut ctx.accounts.escrow;
    require!(amount <= escrow.balance, AmmError::InsufficientBalance);

    escrow.balance = escrow
        .balance
        .checked_sub(amount)
        .ok_or(AmmError::MathOverflow)?;

    **escrow.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

    emit!(EscrowEvent {
        user: ctx.accounts.user.key(),
        action: EscrowAction::Withdraw,
        amount,
        new_balance: escrow.balance,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
```
