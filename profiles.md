# Profiles

User identity and portfolio tracking for connected wallets.

## Username

Users can set a display name on the Profile page (`/profile`).

| Constraint | Value |
|------------|-------|
| Length | 3–20 characters |
| Characters | Alphanumeric + underscore |
| Uniqueness | Must be unique across all users |

Usernames appear on the leaderboard, in chat, and on trade history. If no username is set, the truncated wallet address is shown instead.

## API

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/profile` | GET | Get username for wallet |
| `/api/profile` | PUT | Set or update username (unique constraint) |

## Profile Page

Shows:
- Wallet address
- Summary cards: positions count, total value, P&L, open orders
- Three-tab interface (same as Positions page):
  - **Positions** — open positions with P&L
  - **Open Orders** — pending orders
  - **History** — all trade activity
