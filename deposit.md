# deposit

Deposits SOL from the user's wallet into their per-user escrow PDA. Creates the escrow account on the first call.

**Caller:** User
**Status:** Implemented

## Parameters

| Name | Type | Description |
|---|---|---|
| amount | u64 | Lamports to deposit (must be > 0) |

## Accounts

| Account | Type | Description |
|---|---|---|
| user | Signer, mut | User's wallet |
| escrow | PDA, init_if_needed, mut | User's escrow account — created on first deposit |
| system_program | Program | Solana system program |

## PDA derivation

```
seeds: ["escrow", user.key()]
```

## Logic

1. Validate `amount > 0`
2. Transfer `amount` lamports from user wallet to escrow PDA via CPI
3. Set `escrow.owner` to the user's pubkey
4. Increment `escrow.balance` by `amount` (checked add)
5. Store the PDA bump
6. Emit `EscrowEvent` with action `Deposit`

## Errors

| Error | Condition |
|---|---|
| ZeroAmount | `amount == 0` |
| MathOverflow | Balance addition overflows |

## Source

```rust
pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, AmmError::ZeroAmount);

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        amount,
    )?;

    let escrow = &mut ctx.accounts.escrow;
    escrow.owner = ctx.accounts.user.key();
    escrow.balance = escrow
        .balance
        .checked_add(amount)
        .ok_or(AmmError::MathOverflow)?;
    escrow.bump = ctx.bumps.escrow;

    emit!(EscrowEvent {
        user: ctx.accounts.user.key(),
        action: EscrowAction::Deposit,
        amount,
        new_balance: escrow.balance,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
```
