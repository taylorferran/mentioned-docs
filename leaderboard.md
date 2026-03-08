# Leaderboard

Weekly trader rankings that reset every Monday at 00:00 UTC.

## How It Works

1. When a user trades through Mentioned, the trade is recorded in the `polymarket_trades` database table (wallet, market, amount, side)
2. The leaderboard API (`GET /api/polymarket/leaderboard`) fetches all distinct wallets that traded this week from the DB
3. For each wallet, it calls Jupiter's `/history` endpoint to get all trade events
4. Events are filtered to the current week by timestamp
5. Metrics are computed from Jupiter's event types:
   - **Volume** — sum of `totalCostUsd` from `order_filled` events
   - **P&L** — sum of `realizedPnl` from `payout_claimed` events
   - **Winning trades** — count of claims where `realizedPnl > 0`
   - **Total trades** — count of `order_filled` events
6. Usernames are batch-loaded from `user_profiles`
7. Results are cached for 3 minutes

## UI

### Summary Cards
Top of page shows aggregate stats:
- Total traders this week
- Total volume
- Top P&L
- Best win rate

### Rankings Table
Sortable by **P&L**, **Volume**, or **Win Rate**.

| Column | Description |
|--------|-------------|
| Rank | Position (gold/silver/bronze badges for top 3) |
| Trader | Username or truncated wallet address |
| P&L | Realized profit/loss this week |
| Win Rate | Winning trades / total trades |
| Winning Trades | Count of profitable claims |
| Volume | Total USD volume traded |

## Caching

Results are cached for 3 minutes to avoid excessive Jupiter API calls. Pass `?debug=1` to get raw uncached data for debugging.
