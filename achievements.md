# Achievements

18 achievements that unlock automatically when conditions are met. Each awards bonus points. Achievements are idempotent — duplicate unlocks are silently ignored via a unique constraint on `(wallet, achievement_id)`.

## Profile

| ID | Name | Points | Condition |
|----|------|--------|-----------|
| `set_nickname` | Named & Famed | 75 | Set a username |
| `set_pfp` | Fresh Fit | 50 | Set a profile emoji |

## Trading Basics

| ID | Name | Points | Condition |
|----|------|--------|-----------|
| `first_trade` | First Shot | 150 | Place first Polymarket trade |
| `win_trade` | Winner Winner | 225 | Claim a winning position |
| `lose_trade` | Battle Scarred | 75 | Hold a position that settles as a loss |

## Trade Milestones

| ID | Name | Points | Condition |
|----|------|--------|-----------|
| `10_trades` | Getting Started | 100 | 10 total trades placed |
| `50_trades` | On Fire | 250 | 50 total trades placed |
| `100_trades` | Centurion | 500 | 100 total trades placed |

## Win Milestones

| ID | Name | Points | Condition |
|----|------|--------|-----------|
| `3_wins` | Hat Trick | 150 | 3 winning claims |
| `10_wins` | King of the Hill | 400 | 10 winning claims |

## Chat

| ID | Name | Points | Condition |
|----|------|--------|-----------|
| `first_chat` | Say Something | 50 | Send first chat message |
| `50_chats` | Loud Mouth | 150 | Send 50 chat messages |

## Free Markets

| ID | Name | Points | Condition |
|----|------|--------|-----------|
| `first_free_trade` | Free Player | 75 | Place first free market trade |
| `free_market_win` | Play Money Pro | 150 | Win on a free market |

## How Achievements Unlock

Achievements are checked and awarded inside API route handlers immediately after the triggering action. For example:
- After `POST /api/polymarket/trades/record` → checks trade count milestones
- After `POST /api/chat` → checks chat milestones
- After `POST /api/custom/[id]/trade` → checks free market trade
- After resolution scoring → checks `free_market_win`

All achievement unlocks use `insertPointEvent` with `action = 'achievement'` and call `AchievementContext` to broadcast a toast in the UI.

Discord link is required for achievement points to be awarded (same as all other points).
