# Database Schema

PostgreSQL hosted on Railway.

## Core Tables

### user_profiles

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text (unique) | Wallet address |
| username | text (unique) | Display name |
| pfp_emoji | text | Profile emoji |
| discord_id | text (unique) | Linked Discord user ID |
| discord_username | text (unique) | Discord display name |
| created_at | timestamp | Account creation |
| updated_at | timestamp | Last update |

### chat_messages

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Sender wallet |
| username | text | Sender display name |
| message | text | Content (max 200 chars) |
| created_at | timestamp | When sent |

**Index:** `created_at DESC`

### event_chat_messages

Per-event chat (scoped to a Polymarket event). Includes `pfp_emoji` from profiles.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| event_id | text | Polymarket event ID |
| wallet | text | Sender wallet |
| username | text | Sender display name |
| pfp_emoji | text | Profile emoji |
| message | text | Content |
| created_at | timestamp | When sent |

---

## Polymarket / Trading

### polymarket_trades

Records Polymarket trades for leaderboard and points tracking.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Trader wallet |
| market_id | text | Polymarket market ID |
| event_id | text | Polymarket event ID |
| is_yes | boolean | YES side |
| is_buy | boolean | Buy direction |
| side | text | Human-readable side |
| amount_usd | numeric | Trade amount USD |
| tx_signature | text | Solana tx signature |
| market_title | text | Market display name |
| created_at | timestamp | When recorded |

**Indexes:** `wallet + created_at`, `created_at`

---

## Points & Achievements

### point_events

Points ledger. Append-only. Unique on `(wallet, action, ref_id)` for deduplication.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Earner wallet |
| action | text | Point action type |
| points | integer | Points awarded |
| ref_id | text | Dedup key (tx sig, market ID, etc.) |
| metadata | jsonb | Extra context |
| created_at | timestamp | When earned |

**Actions:** `trade_placed`, `first_trade`, `claim_won`, `chat_message`, `hold_1h`, `hold_4h`, `hold_24h`, `achievement`, `custom_market_win`

### user_achievements

Tracks which achievements each wallet has unlocked. Unique on `(wallet, achievement_id)`.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Wallet |
| achievement_id | text | Achievement slug |
| points_awarded | integer | Bonus points given |
| unlocked_at | timestamp | When unlocked |

---

## Free Markets (Virtual)

### custom_markets

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| title | text | Market title |
| description | text | Description |
| cover_image_url | text | Cover image |
| stream_url | text | Live stream link |
| status | text | `draft`, `open`, `locked`, `resolved` |
| lock_time | timestamp | When trading stops |
| b_parameter | integer | LMSR b param (default 500) |
| play_tokens | integer | Starting balance per user (default 1000) |

### custom_market_words

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | integer | Parent market |
| word | text | Outcome label |
| resolved_outcome | boolean | null = unresolved, true = YES, false = NO |

### custom_market_word_pools

One row per word. LMSR pool state.

| Column | Type | Description |
|--------|------|-------------|
| word_id | integer (PK) | FK to custom_market_words |
| yes_qty | bigint | Net YES shares outstanding |
| no_qty | bigint | Net NO shares outstanding |
| updated_at | timestamp | Last update |

### custom_market_positions

User share holdings per word. Unique on `(word_id, wallet)`.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | integer | Market |
| word_id | integer | Word |
| wallet | text | User wallet |
| yes_shares | bigint | YES shares held |
| no_shares | bigint | NO shares held |
| tokens_spent | bigint | Total tokens spent buying |
| tokens_received | bigint | Total tokens received selling/redeeming |
| updated_at | timestamp | Last update |

### custom_market_balances

Play token balance per user per market. Composite PK on `(market_id, wallet)`.

| Column | Type | Description |
|--------|------|-------------|
| market_id | integer | Market |
| wallet | text | User wallet |
| balance | bigint | Current play token balance |

### custom_market_trades

Full trade log.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | integer | Market |
| word_id | integer | Word |
| wallet | text | Trader |
| action | text | `buy` or `sell` |
| side | text | `YES` or `NO` |
| shares | bigint | Shares traded |
| cost | bigint | Tokens spent or received |
| yes_price | numeric | YES implied price after trade |
| no_price | numeric | NO implied price after trade |
| created_at | timestamp | When traded |

### custom_market_price_history

Price snapshots for charting.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| word_id | integer | Word |
| yes_price | numeric | YES implied price |
| no_price | numeric | NO implied price |
| recorded_at | timestamp | Snapshot time |

---

## On-Chain (Mention Market AMM)

### trade_events

On-chain trades indexed via Helius webhook.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| signature | text | Transaction signature |
| market_id | text | On-chain market ID |
| word_index | integer | Word index in market |
| direction | integer | 0 = YES, 1 = NO |
| is_buy | boolean | Buy or sell |
| quantity | bigint | Token quantity |
| cost | bigint | SOL cost (lamports) |
| fee | bigint | Fee (lamports) |
| new_yes_qty | bigint | YES qty after trade |
| new_no_qty | bigint | NO qty after trade |
| implied_price | bigint | Price after trade |
| trader | text | Trader wallet |
| block_time | timestamp | On-chain block time |
| created_at | timestamp | When indexed |

**Indexes:** unique `(signature, market_id, word_index, trader)`, `market_id + block_time`, `trader + block_time`, `market_id + word_index`

### market_transcripts

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | text (unique) | Market ID |
| transcript | text | Transcript content |
| source_url | text | Source |
| submitted_by | text | Submitter wallet |
| created_at | timestamp | When submitted |

### market_metadata

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | text (unique) | Market ID |
| image_url | text | Cover image URL |
| created_at | timestamp | When added |

### event_streams

| Column | Type | Description |
|--------|------|-------------|
| event_id | text (unique) | Event ID |
| stream_url | text | Live stream URL |
| updated_at | timestamp | Last update |
