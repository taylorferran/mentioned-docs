# Discord Integration

Discord linking connects a wallet to a Discord identity. It is the primary mechanism for sybil resistance and is required to earn any points on the platform.

## Why Discord

- **Sybil resistance** — One Discord account can only link to one wallet. Prevents farming points with multiple wallets.
- **Points gate** — All point awards (`insertPointEvent`) silently no-op if the wallet has no linked Discord.
- **Verified role** — Linked users receive a verified badge in the Mentioned Discord guild.

## OAuth Flow

1. User clicks "Link Discord" on their profile
2. Frontend redirects to Discord OAuth with `state = base64url({ wallet })`
3. Discord redirects to `GET /api/discord/callback?code=...&state=...`
4. Backend decodes state to get the wallet
5. Exchange `code` for Discord access token
6. Fetch Discord user info (`GET /users/@me`)
7. **Sybil check** — Reject if that `discord_id` is already linked to a different wallet
8. Store `discord_id` and `discord_username` in `user_profiles`
9. Add user to the Discord guild (if not already a member)
10. Assign the verified role (`DISCORD_VERIFIED_ROLE_ID`)

## API Routes

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/discord/link` | GET | Get current Discord link status for wallet |
| `/api/discord/link` | POST | Initiate link (returns OAuth URL) |
| `/api/discord/link` | DELETE | Unlink Discord from wallet |
| `/api/discord/callback` | GET | OAuth callback handler |
| `/api/discord/unlink` | DELETE | Alias for unlink |

## Sybil Protection

The unique constraint on `discord_id` in `user_profiles` enforces one-to-one mapping at the database level. The callback handler also checks explicitly before inserting, returning a 409 error if the Discord account is already used.

## Points Gate

Every call to `insertPointEvent` first calls `hasDiscordLinked(wallet)`. If the wallet has no Discord linked, the function returns `null` and no point row is inserted. This applies to:
- All Polymarket trade points
- All chat points
- All free market points
- All achievement unlocks

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `DISCORD_CLIENT_ID` | Discord OAuth app ID |
| `DISCORD_CLIENT_SECRET` | Discord OAuth app secret |
| `DISCORD_BOT_TOKEN` | Bot token for guild management |
| `DISCORD_GUILD_ID` | Server ID to add users to |
| `DISCORD_VERIFIED_ROLE_ID` | Role ID to assign on link |
