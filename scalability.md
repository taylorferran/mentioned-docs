# Scalability Roadmap

The current implementation works for a low-traffic devnet prototype. This page documents the architectural bottlenecks and planned fixes for production scaling.

## 1. ~~Move trade history to an indexer~~ — DONE

Implemented. Helius webhook captures `TradeEvent` logs in real-time, stores them in Railway Postgres, and serves them via REST API (`/api/trades`, `/api/trades/chart`). Frontend now makes a single HTTP call instead of 5-15 RPC calls.

See [Indexer](indexer.md) for the full implementation.

## 2. Replace client-side account fetching with an API layer

**Problem**: `fetchAllMarkets()` calls `getProgramAccounts` with a discriminator filter on every call. Gets slower as more markets are created. Most RPC providers rate-limit or charge heavily for `getProgramAccounts`.

`fetchUserPositions` fetches all token accounts then cross-references against all market mint PDAs — multiple RPC calls per market.

**Fix**:
- Index `MarketAccount` state changes via Geyser/webhooks into a database
- Serve market lists, search, and filtering from an API
- Index token balances per user so position lookups are a single DB query
- Cache market data with short TTLs (5–15s)

## 3. Replace 15s polling with WebSocket subscriptions

**Problem**: The market page polls every 15 seconds via `setInterval`. Stale prices for up to 15s, unnecessary RPC calls when nothing changes, scales linearly with connected clients.

**Fix**:
- Use Solana `accountSubscribe` WebSocket for real-time `MarketAccount` updates
- Use indexer WebSocket/SSE feed for new trade events
- Push updates from a backend service via WebSocket to all connected frontends

## 4. Server-side LMSR math / price API

**Problem**: LMSR calculations are duplicated between on-chain Rust (fixed-point `i128` at 1e9) and client-side JavaScript (floating-point `Math.exp`/`Math.log`). Potential precision drift between UI preview and contract execution.

**Fix**:
- Expose a price API from the backend using the same fixed-point math (or a Rust WASM module)
- Cache current prices, update on each trade event
- Frontend reads prices from the API instead of recomputing

## 5. Move market metadata off-chain

**Problem**: Market labels, word labels, and metadata stored on-chain waste rent and limit content (64/32 char caps). Hard to add rich metadata (descriptions, images, categories).

**Fix**:
- Store only a content hash or URI on-chain
- Keep full metadata in a database or decentralized storage (Arweave, IPFS, or Postgres)
- Frontend fetches metadata from the API, validates against on-chain hash

## 6. Server-side cost basis

**Problem**: Profile page computes cost basis by fetching entire trade history client-side, building a `costBasisMap` in `useMemo`. Gets slower as users accumulate trades, recomputes on every render.

**Fix**:
- Indexer maintains running cost basis per `(user, market, word, side)` as trades are indexed
- Profile API returns positions with pre-computed cost basis, average price, and P&L

## 7. getProgramAccounts filters

**Short-term improvement** before a full indexer: add `memcmp` filters to narrow `getProgramAccounts` results:
- Filter markets by status (Open only for homepage)
- Filter by `resolves_at` range for upcoming/past markets
- Reduces data transferred and deserialization overhead

## 8. Pagination

**Problem**: All list views fetch everything at once with no pagination. Works with a handful of markets, breaks with hundreds.

**Fix**:
- API-level pagination with cursor-based or offset/limit patterns
- Frontend infinite scroll or paginated tables
- Trade history needs pagination — currently limited to 1000 signatures max

## 9. Caching layer

Add caching between the frontend and data sources:
- **CDN/edge cache** for market lists and metadata (5–15s TTL)
- **Redis/in-memory cache** for computed prices, LP positions, aggregate stats
- **Stale-while-revalidate** pattern for instant UI even with slightly stale data

## Migration priority

| Priority | Change | Effort | Impact |
|---|---|---|---|
| ~~**P0**~~ | ~~Indexer for trade events~~ | ~~Medium~~ | **DONE** — see [Indexer](indexer.md) |
| **P0** | API layer for markets/positions | Medium | Eliminates `getProgramAccounts` bottleneck |
| **P1** | WebSocket subscriptions | Low | Real-time prices, eliminates polling |
| **P1** | Server-side cost basis | Low | Fast profile loads, accurate P&L |
| **P2** | Price API with fixed-point math | Low | Precision parity with contract |
| **P2** | Off-chain metadata | Medium | Cheaper markets, richer content |
| **P3** | Pagination everywhere | Low | Required for >50 markets |
| **P3** | Caching layer | Medium | Production-grade performance |
