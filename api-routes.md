# API Routes

All API routes are Next.js API route handlers under `/api/`.

## Polymarket (Jupiter Proxy)

All Polymarket routes proxy Jupiter's Prediction API (`https://api.jup.ag/prediction/v1`) with API key authentication and client IP forwarding.

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/polymarket` | GET | Fetch events by category (default: esports) |
| `/api/polymarket/event` | GET | Fetch single event with markets (`?eventId=`) |
| `/api/polymarket/positions` | GET | Fetch user positions (`?ownerPubkey=`) |
| `/api/polymarket/positions/close` | DELETE | Close a position (returns unsigned tx) |
| `/api/polymarket/positions/claim` | POST | Claim settled position payout (returns unsigned tx) |
| `/api/polymarket/orders` | POST | Create a new order |
| `/api/polymarket/orders/list` | GET | Fetch user's open orders (`?ownerPubkey=`) |
| `/api/polymarket/orderbook` | GET | Fetch market orderbook (`?marketId=`) |
| `/api/polymarket/history` | GET | Fetch trade history (`?ownerPubkey=`) |
| `/api/polymarket/leaderboard` | GET | Compute weekly leaderboard (3-min cache) |
| `/api/polymarket/trades/record` | POST | Record trade to DB for leaderboard tracking |

## User & Chat

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/profile` | GET | Get username for wallet |
| `/api/profile` | PUT | Set/update username (unique constraint) |
| `/api/chat` | GET | Fetch recent messages (50 max, `?after=` for polling) |
| `/api/chat` | POST | Send message (200 char max, 500ms rate limit) |

## Mention Markets (Future)

On-chain data routes for custom mention markets. These are used by the `/markets` and `/market/[id]` pages.

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/trades` | GET | Fetch on-chain trades (`?marketId=` or `?trader=`) |
| `/api/trades/chart` | GET | Fetch trade data for charting (market + word) |
| `/api/trades/volume` | GET | Volume totals for markets (`?marketIds=`) |
| `/api/market-image` | GET | Fetch market cover images |
| `/api/transcript` | GET/POST | Store/fetch event transcripts |
| `/api/webhook` | POST | Helius webhook for indexing on-chain events |

## Other

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/waitlist` | POST | Email signup |
