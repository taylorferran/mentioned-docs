# Profiles

User identity, stats, and achievement showcase.

## Profile Data

Stored in `user_profiles`:

| Field | Constraint | Description |
|-------|-----------|-------------|
| username | 3–20 chars, alphanumeric + underscore, unique | Display name across leaderboard, chat, and history |
| pfp_emoji | any emoji | Profile picture shown in chat and on profile |
| discord_id | unique | Linked Discord account (required for points) |
| discord_username | unique | Discord display name |

If no username is set, the truncated wallet address is shown instead.

## Profile Page (`/profile/[username]`)

Public profile showing:
- Wallet address
- Summary cards: positions count, total value, P&L, open orders
- Trade activity tab
- Unlocked achievements with badges

## API

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/profile` | GET | Get profile for connected wallet |
| `/api/profile` | POST | Create profile |
| `/api/profile/[username]` | GET | Fetch public profile by username |

## Achievements

Achievement unlock status is shown on the profile. Each badge displays the achievement name, description, and points awarded. Locked achievements are shown as grayed out. See [Achievements](achievements.md).

## Discord Linking

The profile page includes a Discord link/unlink control. Linking Discord is required to earn points and appear on the points leaderboard. See [Discord Integration](discord-integration.md).
