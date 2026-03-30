# Leaderboard

The leaderboard at `/leaderboard` has two tabs: **Points** and **Trading**. Both reset weekly on Monday at 00:00 UTC.

## Points Leaderboard

Rankings by points earned through trading, winning, chatting, and holding. Only wallets with a linked Discord account appear here.

**Endpoint:** `GET /api/polymarket/leaderboard/points?period=weekly|alltime`

**How it works:**
1. Aggregate `point_events` rows since Monday 00:00 UTC (or all-time)
2. Group by wallet, sum points
3. Batch-load usernames from `user_profiles`
4. Return sorted by total points

**Breakdown shown per user:**
- Total points
- Points from trades
- Points from wins
- Points from chat
- Points from holds

See [Points System](points.md) for all point actions and values.

## Trading Leaderboard

Rankings by Polymarket trading performance: P&L, volume, win rate.

**Endpoint:** `GET /api/polymarket/leaderboard?period=weekly`

**How it works:**
1. Fetch distinct wallets that traded this week from `polymarket_trades`
2. For each wallet, call Jupiter's `/history` endpoint
3. Filter events to the current week by timestamp
4. Compute metrics from Jupiter event types:
   - **Volume** — sum of `totalCostUsd` from `order_filled` events
   - **P&L** — sum of `realizedPnl` from `payout_claimed` events
   - **Winning trades** — count of claims where `realizedPnl > 0`
   - **Total trades** — count of `order_filled` events
5. Load usernames from `user_profiles`
6. Cache results for 3 minutes

Pass `?debug=1` to bypass the cache and get raw data.

## UI

### Summary Cards
- Total traders this week
- Total volume
- Top P&L
- Best win rate

### Rankings Table

| Column | Description |
|--------|-------------|
| Rank | Position (gold/silver/bronze badges for top 3) |
| Trader | Username or truncated wallet |
| Points / P&L | Depending on active tab |
| Win Rate | Winning trades / total trades |
| Volume | Total USD volume (trading tab) |
