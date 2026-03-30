# Platform Overview

Mentioned is a social prediction market platform on Solana. It combines two market types (free/virtual and real-money/Polymarket), a points system gated behind Discord, and competitive social features.

## Pages

### Homepage (`/`)
Landing page with animations and hero content. Entry point for new users.

### Markets (`/markets`)
Polymarket mention market events. Each card shows outcome odds, volume, and close time.

### Event Detail (`/polymarkets/event/[eventId]`)
Full trading interface for a Polymarket event. Shows:
- Multiple markets per event (e.g. team matchups)
- Live orderbook (YES/NO depth visualization)
- Trading panel (place orders)
- Per-event chat
- User positions and open orders
- Settlement countdown

### Free Market Detail (`/custom/[id]`)
Virtual LMSR market trading interface. Shows:
- Word grid with YES/NO prices
- Buy/sell panel with play token denomination
- LMSR price chart
- User positions and trade history

### Leaderboard (`/leaderboard`)
Two tabs:
- **Points** — Weekly/all-time rankings by points (Discord-linked users only)
- **Trading** — P&L, volume, win rate rankings

### Positions (`/positions`)
Connected wallet's Polymarket positions, open orders, and trade history across three tabs.

### Profile (`/profile/[username]`)
Public profile showing stats, active positions, trade activity, and unlocked achievements.

### Admin (`/customadmin`)
Admin interface for creating and managing free/virtual markets: create, lock, resolve words, view LP positions.

### Admin (`/admin`)
Admin interface for on-chain Mention Market: escrow management, market creation, liquidity, resolution.

### Waitlist (`/waitlist`)
Email signup form.

## Components

| Component | Purpose |
|-----------|---------|
| `Header` | Navigation, wallet connect, mode toggle |
| `Footer` | Site links, social, legal |
| `ConnectModal` | Phantom / Privy wallet connection modal |
| `GlobalChat` | Collapsible chat widget with unread badge |
| `EventChat` | Per-event chat (with pfp emoji) |
| `OrderBook` | YES/NO orderbook with bar chart depth |
| `EventPriceChart` | Recharts-based price visualization |
| `MarketCard` | Polymarket event preview card |
| `CustomEventCard` | Free market preview card |
| `QuickBuy` | Fast trade entry |
| `TradingChart` | Detailed price/volume charts |
| `SharePnLModal` | Shareable P&L image export |
| `DepositModal` | SOL escrow deposit/withdraw |
| `FlashValue` | Animated number transitions |
| `Ticker` | Live price ticker |
| `TradeTicker` | Recent trades feed |
| `CountdownTimer` | Live countdown to market close |
| `ResolveRules` | Market resolution explanation |
| `WalletProviderWrapper` | Root-level Privy + Phantom context |

## Auth & Wallet

Mentioned supports two wallet connection methods via `WalletContext`:

| Method | Notes |
|--------|-------|
| **Phantom** | Wallet Standard — `solana:signAndSendTransaction` |
| **Privy** | Passwordless auth with embedded Solana wallet |

On connect:
- Balance polled every 10 seconds via Helius RPC
- Profile (username, pfpEmoji) fetched and cached
- Wallet events listened for account changes

### Admin Auth
Simple hardcoded wallet check in `lib/adminAuth.ts`. Admins are a fixed list of approved wallet addresses.
