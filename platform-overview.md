# Platform Overview

Mentioned is a social prediction market platform on Solana. The current product integrates Jupiter's Polymarket API for esports and live event trading, with competitive features (leaderboards, profiles, chat) built on top.

## Pages

### Homepage (`/`)
Polymarket esports events in a grid layout. Events are split into **Live Now** and **Upcoming** sections. Each card shows team odds (YES/NO percentages), volume, and close time. Data fetched from `/api/polymarket?category=esports`.

### Event Detail (`/polymarkets/event/[eventId]`)
Full trading interface for a Polymarket event. Shows multiple markets per event (e.g. team matchups), live orderbook visualization with YES/NO sides, trading panel, current positions, order history, and settlement countdown.

### Positions (`/positions`)
Three-tab interface for the connected wallet:
- **Positions** — Open positions with unrealized P&L, mark price, avg price, payout if right, estimated settlement time. **Close** (red) or **Claim** (green) buttons depending on settlement status.
- **Open Orders** — Pending orders with side, contracts, max price, size, creation time.
- **History** — All trade history: fills, settlements, claims, failures. Shows action, status, price, deposit/withdraw amounts, realized P&L, fees.

### Leaderboard (`/leaderboard`)
Weekly trader rankings resetting every Monday UTC. Summary cards: total traders, total volume, top P&L, best win rate. Sortable by P&L, Volume, or Win Rate. See [Leaderboard](leaderboard.md).

### Profile (`/profile`)
Username setup (3–20 chars, alphanumeric + underscore, unique). Shows wallet address and summary cards (positions, total value, P&L, open orders). Includes the same three-tab positions/orders/history interface. See [Profiles](profiles.md).

### Mention Markets (`/markets`) — Future
Lists on-chain mention markets (native LMSR protocol). Filter tabs for Active and Resolved markets. Each card shows word grid with YES/NO prices, category, title, event countdown, and volume. See [Contract Overview](contract-overview.md).

### Market Detail (`/market/[id]`) — Future
On-chain trading interface for mention markets. Buy/sell with denomination toggle (Shares/USD/SOL). LMSR price chart, word selection, trade history, user positions.

### Waitlist (`/waitlist`)
Email signup form.

## Components

| Component | Purpose |
|-----------|---------|
| `Header` | Logo, nav (Leaderboard, Positions), wallet dropdown (Profile, Disconnect) |
| `Footer` | Site links (Waitlist, Discord, Twitter), legal disclaimer |
| `GlobalChat` | Collapsible chat widget with polling, optimistic sends, unread badge |
| `OrderBook` | YES/NO orderbook visualization with bar charts |
| `MarketCard` | Market preview card with image, word grid, odds, volume |
| `MarketChart` | LMSR price history chart (multi-word, color-coded) |
| `FlashValue` | Animated value change transitions |
| `CountdownTimer` | Real-time event countdown |
| `DepositModal` | SOL deposit/withdraw modal |
| `QuickBuy` | Quick trade execution |
| `SharePnLModal` | Social sharing of P&L results |
| `WalletProviderWrapper` | Root-level wallet context provider |

## Wallet Integration

Uses Wallet Standard (`@wallet-standard/app`) with Phantom wallet.

- **Auto-reconnect** — On page load, checks if Phantom has a cached account and silently connects
- **Balance polling** — Every 10 seconds via `@solana/kit` RPC client
- **Transaction signing** — Phantom's `solana:signAndSendTransaction` feature, transaction encoded as Uint8Array (base64 from API → bytes)
- **Account change detection** — Listens for wallet events to update connected account
