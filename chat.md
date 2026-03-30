# Chat

Two chat systems exist: global chat (site-wide) and per-event chat.

## Global Chat

Available on every page via the collapsible chat bubble in the bottom-right corner.

- **Polling** — Fetches new messages every 3 seconds via `GET /api/chat?after={lastMessageId}`
- **Rate limit** — 500ms per wallet
- **Message limit** — 200 characters
- **Usernames** — Pulled from `user_profiles` or truncated wallet address
- **Optimistic UI** — Message appears instantly with a temporary negative ID, replaced on server confirmation
- **Unread badge** — Shows count of new messages since last open
- **Points** — Each message awards 2 points (daily cap: 10), Discord link required

### API

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/chat` | GET | Recent messages (50 max, `?after=` for polling) |
| `/api/chat` | POST | Send message (awards chat points + checks achievements) |

## Per-Event Chat

Each event detail page (`/polymarkets/event/[eventId]`) has its own chat scoped to that event. Messages include `pfp_emoji` from the sender's profile.

### API

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/chat/event` | GET | Fetch messages for an event (`?eventId=`) |
| `/api/chat/event` | POST | Send a message to an event chat |

## Achievements

Sending the first chat message unlocks `first_chat` (50 pts). Sending 50 messages unlocks `loud_mouth` (150 pts). These are checked on every `POST /api/chat`.
