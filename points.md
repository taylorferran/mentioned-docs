# Points System

Points reward engagement across trading, winning, chatting, and holding. **A linked Discord account is required to earn any points.** This is the primary sybil resistance mechanism.

## Earning Points

| Action | Points | Cap | Condition |
|--------|--------|-----|-----------|
| `trade_placed` | 10 | 20/day | Place a Polymarket trade (min $1 USD) |
| `first_trade` | 100 | one-time | First ever Polymarket trade |
| `claim_won` | 50 | — | Claim a winning Polymarket position |
| `chat_message` | 2 | 10/day | Send a chat message |
| `hold_1h` | 5 | — | Hold a position for 1 hour |
| `hold_4h` | 15 | — | Hold a position for 4 hours |
| `hold_24h` | 30 | — | Hold a position for 24 hours |
| `achievement` | varies | — | Unlock an achievement (see [Achievements](achievements.md)) |
| `custom_market_win` | 0.5× net profit | — | Win on a free market |

### Free Market Scoring
Points from free markets = `max(0, floor((tokens_received - tokens_spent) * 0.5))`. Only awarded if net profit > 0.

### Trade Point Guards

Two checks run before awarding `trade_placed` points in `POST /api/polymarket/trades/record`:

1. **Minimum amount** — `amountUsd < 1` skips the award (no points for dust trades)
2. **Daily cap** — `getTradePointsCountToday(wallet)` counts `trade_placed` events since midnight UTC; skips if already at 20 awards today

Both guards run before `awardPoints` is called. The daily count query mirrors the same pattern used for `getChatPointsCountToday` (chat's 10/day cap).

### Hold Points
Hold duration is computed from the earliest `created_at` in `polymarket_trades` for a given `wallet + market_id`. Each hold tier is a one-time award per position.

## Discord Requirement

Points are only inserted if the wallet has a linked Discord account. Every `insertPointEvent` call first checks `hasDiscordLinked(wallet)` — if not linked, the call returns `null` silently. See [Discord Integration](discord-integration.md).

## Deduplication

Point events use a unique constraint on `(wallet, action, ref_id)`. Duplicate inserts are silently ignored, making all point actions safe to retry.

## Leaderboard

Points are aggregated into weekly (since Monday 00:00 UTC) and all-time totals from the `point_events` table. The leaderboard at `/leaderboard` shows both views. See [Leaderboard](leaderboard.md).

## Database

All point events are stored in the `point_events` table:

| Column | Description |
|--------|-------------|
| wallet | Earner's wallet address |
| action | Point action type (trade_placed, claim_won, etc.) |
| points | Points awarded |
| ref_id | Deduplication key (e.g. tx signature, market ID) |
| metadata | JSONB — extra context (e.g. amount, market title) |
| created_at | When earned |
