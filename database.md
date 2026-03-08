# Database Schema

PostgreSQL hosted on Railway.

## polymarket_trades

Records trades placed through Mentioned for leaderboard tracking.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Trader's wallet address |
| market_id | text | Polymarket market ID |
| event_id | text | Polymarket event ID |
| is_yes | boolean | YES or NO side |
| is_buy | boolean | Buy or sell |
| side | text | Human-readable side label |
| amount_usd | numeric | Trade amount in USD |
| tx_signature | text | Solana transaction signature |
| created_at | timestamp | When the trade was recorded |

**Indexes:** `wallet + created_at`, `created_at`

## user_profiles

User identity and settings.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Wallet address (unique) |
| username | text | Display name (unique) |
| created_at | timestamp | Account creation |
| updated_at | timestamp | Last update |

## chat_messages

Global chat messages.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| wallet | text | Sender's wallet |
| username | text | Sender's display name |
| message | text | Message content (max 200 chars) |
| created_at | timestamp | When sent |

**Indexes:** `created_at DESC`

## trade_events (Mention Markets — Future)

On-chain mention market trades indexed via Helius webhook.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| signature | text | Transaction signature |
| market_id | text | Market ID |
| word_index | integer | Word index in market |
| direction | integer | 0 = YES, 1 = NO |
| is_buy | boolean | Buy or sell |
| quantity | bigint | Token quantity |
| cost | bigint | SOL cost in lamports |
| fee | bigint | Fee in lamports |
| new_yes_qty | bigint | YES quantity after trade |
| new_no_qty | bigint | NO quantity after trade |
| implied_price | bigint | Price after trade |
| trader | text | Trader's wallet |
| block_time | timestamp | On-chain block time |
| created_at | timestamp | When indexed |

**Indexes:** `signature` (unique composite with market_id, word_index, trader), `market_id + block_time`, `trader + block_time`, `market_id + word_index`

## market_transcripts

Event transcript text for markets.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | text | Market ID (unique) |
| transcript | text | Transcript content |
| source_url | text | Source URL |
| submitted_by | text | Submitter wallet |
| created_at | timestamp | When submitted |

## market_metadata

Market cover images.

| Column | Type | Description |
|--------|------|-------------|
| id | serial | Primary key |
| market_id | text | Market ID (unique) |
| image_url | text | Cover image URL |
| created_at | timestamp | When added |
