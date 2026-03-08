# Chat

Global real-time chat available to all connected users.

## How It Works

- **Polling-based** — fetches new messages every 3 seconds via `GET /api/chat?after={lastMessageId}`
- **Messages** — limited to 200 characters, rate-limited to 500ms per wallet
- **Usernames** — auto-populated from `user_profiles` or truncated wallet address
- **Optimistic UI** — messages appear instantly with a temporary negative ID, replaced when the server confirms

## UI

Collapsible chat bubble in the bottom-right corner of every page.

- Unread badge shows count of new messages since last open
- Auto-scrolls to bottom when expanded
- Shows username, message, and relative timestamp

## API

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/chat` | GET | Fetch recent messages (50 max, `?after=` for polling) |
| `/api/chat` | POST | Send message (200 char max, 500ms rate limit per wallet) |

## Database

Messages stored in `chat_messages` table. See [Database Schema](database.md).
