# Polymarket Integration

All Polymarket trading on Mentioned is proxied through Jupiter's Prediction API (`https://api.jup.ag/prediction/v1`). The frontend never calls Jupiter directly — all requests go through our Next.js API routes which attach the API key and forward the client IP.

> Currently fetching esports markets as a temporary category while we wait for Jupiter to add mention markets to their API. Once available, the platform will switch to mention markets as the primary content.

## Trading Flow

1. User browses events on the homepage or event detail page
2. Selects a market and side (YES/NO), enters amount
3. Frontend calls `POST /api/polymarket/orders` → proxied to Jupiter API
4. Jupiter returns an unsigned Solana transaction
5. Phantom wallet prompts user to sign and send
6. Trade is recorded to `polymarket_trades` table via fire-and-forget POST to `/api/polymarket/trades/record`
7. Position appears in Positions tab; can be closed or claimed after settlement

## API Routes

All routes proxy Jupiter with API key auth and IP forwarding.

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
| `/api/polymarket/leaderboard` | GET | Compute weekly leaderboard (3-min cache, `?debug=1` for raw data) |
| `/api/polymarket/trades/record` | POST | Record trade to DB for leaderboard tracking |

## Position Lifecycle

### Open Position
User has bought YES or NO on a market that hasn't settled yet. Shows unrealized P&L based on current mark price.

### Close
User can close (sell) a position before settlement. Calls `DELETE /api/polymarket/positions/close` → returns unsigned tx → Phantom signs.

### Claim
After a market settles, winning positions show a green **Claim** button. Calls `POST /api/polymarket/positions/claim` → returns unsigned tx → Phantom signs → payout received.

## Event Display

Events are fetched from Jupiter and displayed in two sections:
- **Live Now** — Events with active trading, sorted by volume
- **Upcoming** — Events that haven't started yet, sorted by start time

Each event card shows:
- Team names / matchup
- YES/NO odds (percentages)
- Volume traded
- Close time / countdown

## Orderbook

The event detail page shows a live orderbook for each market. YES orders on one side, NO on the other, with bar chart visualization showing depth at each price level.
