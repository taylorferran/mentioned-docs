# Paid Markets

Paid markets are real-money prediction markets powered by Polymarket via Jupiter's Prediction API. Mentioned acts as a frontend — all order execution, pricing, and settlement is handled by Jupiter/Polymarket.

> **Note:** The platform currently shows esports markets. This is temporary while we wait for Jupiter to add mention markets to their API. Once available, Mentioned will switch to mention market trading as its primary focus.

## Overview

| Property | Value |
|----------|-------|
| Currency | USDC (real money) |
| Pricing | Polymarket order book |
| Settlement | Automatic via Polymarket |
| Points earned | 10 pts per trade (Discord linked, min $1 USD) |

## Trading Flow

1. User browses events on `/markets` or `/polymarkets/event/[eventId]`
2. Selects a market and side (YES/NO), enters amount
3. Frontend calls `POST /api/polymarket/orders` → proxied to Jupiter API
4. Jupiter returns an unsigned Solana transaction
5. Phantom or Privy wallet signs and sends the transaction
6. Trade recorded to `polymarket_trades` for points/leaderboard tracking
7. Position appears in `/positions`; close or claim after settlement

## Position Lifecycle

### Open
Active position with unrealized P&L. Shows mark price, avg price, payout if right, estimated settlement time.

### Close
User sells a position before settlement. Calls `DELETE /api/polymarket/positions/close` → returns unsigned tx → user signs.

### Claim
After market settles, winning positions show a green **Claim** button. Calls `POST /api/polymarket/positions/claim` → returns unsigned tx → user signs.

## API Routes

All routes proxy Jupiter with API key auth and client IP forwarding.

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/polymarket` | GET | List events by category (default: esports) |
| `/api/polymarket/event` | GET | Single event with markets (`?eventId=`) |
| `/api/polymarket/positions` | GET | User's open positions (`?ownerPubkey=`) |
| `/api/polymarket/positions/close` | DELETE | Close a position (returns unsigned tx) |
| `/api/polymarket/positions/claim` | POST | Claim settled payout (returns unsigned tx) |
| `/api/polymarket/orders` | POST | Place an order |
| `/api/polymarket/orders/list` | GET | User's open orders (`?ownerPubkey=`) |
| `/api/polymarket/orderbook` | GET | Market orderbook (`?marketId=`) |
| `/api/polymarket/history` | GET | Trade history (`?ownerPubkey=`) |
| `/api/polymarket/trades/record` | POST | Record trade for points/leaderboard |
| `/api/polymarket/leaderboard` | GET | Trading leaderboard (3-min cache) |
| `/api/polymarket/leaderboard/points` | GET | Points leaderboard |

## Trade Recording

When an order is placed, `POST /api/polymarket/trades/record` is called fire-and-forget to:
- Insert into `polymarket_trades` table
- Award `trade_placed` points (10 pts, daily cap 20, min $1 USD)
- Award `first_trade` points (100 pts, one-time)
- Check and unlock trade count achievements (10/50/100 trades)

Points are only awarded if the wallet has a linked Discord account. See [Points System](points.md).

## Event Display

Events are split into:
- **Live Now** — Active markets with ongoing trading, sorted by volume
- **Upcoming** — Markets not yet open, sorted by start time

Each event card shows team names / outcomes, YES/NO odds, volume, and a countdown to close.
