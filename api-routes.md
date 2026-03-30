# API Routes

All routes are Next.js route handlers under `/app/api/`.

## Polymarket (Jupiter Proxy)

Proxies Jupiter's Prediction API (`https://api.jup.ag/prediction/v1`) with API key auth and client IP forwarding.

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/polymarket` | GET | List mention market events |
| `/api/polymarket/event` | GET | Single event with markets (`?eventId=`) |
| `/api/polymarket/positions` | GET | User's positions (`?ownerPubkey=`) |
| `/api/polymarket/positions/close` | DELETE | Close position (returns unsigned tx) |
| `/api/polymarket/positions/claim` | POST | Claim settled payout (returns unsigned tx) |
| `/api/polymarket/orders` | POST | Place an order |
| `/api/polymarket/orders/list` | GET | User's open orders (`?ownerPubkey=`) |
| `/api/polymarket/orderbook` | GET | Market orderbook (`?marketId=`) |
| `/api/polymarket/history` | GET | Trade history (`?ownerPubkey=`) |
| `/api/polymarket/prices` | GET | Current market prices |
| `/api/polymarket/trades/record` | POST | Record trade for points + leaderboard |
| `/api/polymarket/leaderboard` | GET | Trading leaderboard (3-min cache, `?debug=1` for raw) |
| `/api/polymarket/leaderboard/points` | GET | Points leaderboard (`?period=weekly\|alltime`) |

## Free Markets (Virtual LMSR)

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/custom` | GET | List all custom markets |
| `/api/custom` | POST | Create market (admin only) |
| `/api/custom/[id]` | GET | Get market details |
| `/api/custom/[id]/trade` | POST | Execute buy/sell (LMSR, play tokens) |
| `/api/custom/[id]/positions` | GET | User positions in market |
| `/api/custom/[id]/trades` | GET | Trade history for market |
| `/api/custom/[id]/chart` | GET | Price history for charting |
| `/api/custom/[id]/resolve` | POST | Resolve words and market (admin only) |
| `/api/custom/[id]/words` | GET | Get words for market |
| `/api/custom/[id]/words` | POST | Add words to market |
| `/api/custom/[id]/status` | GET/POST | Get or update market status |

## Profile & Auth

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/profile` | GET | Get profile for wallet |
| `/api/profile` | POST | Create profile |
| `/api/profile/[username]` | GET | Public profile by username |
| `/api/discord/link` | GET | Check Discord link status |
| `/api/discord/link` | POST | Get OAuth URL to initiate linking |
| `/api/discord/link` | DELETE | Unlink Discord |
| `/api/discord/callback` | GET | OAuth callback (exchanges code, assigns role) |
| `/api/discord/unlink` | DELETE | Unlink Discord (alias) |

## Chat

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/chat` | GET | Recent messages (50 max, `?after=` for polling) |
| `/api/chat` | POST | Send message (2 pts, checks achievements) |
| `/api/chat/event` | GET | Per-event messages (`?eventId=`) |
| `/api/chat/event` | POST | Send per-event message |

## On-Chain (Mention Market AMM)

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/trades` | GET | On-chain trades (`?marketId=` or `?trader=`) |
| `/api/trades/chart` | GET | Price history for market+word |
| `/api/trades/volume` | GET | Volume totals (`?marketIds=`) |
| `/api/webhook` | POST | Helius webhook — indexes on-chain trade events |

## Utilities

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/achievements` | GET | All achievement definitions |
| `/api/market-image` | GET | Market cover image |
| `/api/transcript` | GET/POST | Store/fetch event transcripts |
| `/api/streams` | GET | Live stream URLs for events |
| `/api/sitemap` | GET | SEO sitemap |
| `/api/waitlist` | POST | Email signup |
