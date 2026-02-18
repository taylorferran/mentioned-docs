# Repo Audit Report

Security, scalability, and cleanup findings across the contract and frontend codebases.

---

## Security Risks

### Contract — mention-market (old CLOB)

| Severity | Issue | Location |
|---|---|---|
| **HIGH** | `settle_match` backend signer is unchecked | `settle_match.rs:9` |

`backend: Signer<'info>` has no validation that it matches an authorized backend pubkey stored in market state. Anyone can call `settle_match` to match two arbitrary users' escrows without their consent. The `yes_buyer` and `no_buyer` are `UncheckedAccount`, not `Signer`, so the actual traders never authorize the match.

**Fix:** Store an authorized backend pubkey in `WordMarket` state and add a constraint: `constraint = word_market.backend == backend.key()`.

| Severity | Issue | Location |
|---|---|---|
| **MEDIUM** | `claim` instruction has loose market validation | `claim.rs` |

`word_market` is `#[account(mut)]` with no seeds or `has_one` constraint. While exploitability is limited by token mint checks, an explicit PDA constraint would be safer.

> **Note:** The old CLOB contract is being replaced by the AMM. These issues are documented for completeness but the priority is fixing the active AMM contract.

---

### Contract — mention-market-amm (active)

| Severity | Issue | Location | Status |
|---|---|---|---|
| **HIGH** | `u64` to `i64` cast can silently wrap | `math.rs:179` | Open |
| **HIGH** | No fee withdrawal mechanism | — | Open |
| **MEDIUM** | `binary_lmsr_cost` can pass 0 to `fp_ln` | `math.rs:152-154` | Open |
| **MEDIUM** | LMSR saturation on extreme quantities | `math.rs` | Open |
| **MEDIUM** | `implied_price` can exceed `PRECISION` | `math.rs:245` | Open |

#### u64 to i64 silent overflow

`q_yes.checked_add(amount as i64)` — the `as i64` cast wraps without error if `amount > i64::MAX` (9.2e18). While this is an astronomical token quantity, Rust's `as` silently wraps rather than erroring.

**Fix:** Use `i64::try_from(amount).map_err(|_| AmmError::MathOverflow)?` instead.

#### No fee withdrawal

`accumulated_fees` is incremented on every buy/sell but no instruction exists to withdraw them. Fees are permanently locked in the market account. The design doc (`amm_v1_design.md:98`) says "withdrawable by protocol" but no instruction was implemented.

**Fix:** Add a `withdraw_fees` instruction gated by `market.authority`.

#### binary_lmsr_cost zero input

If both `fp_exp` calls return 0 (very negative quantities), `sum = 0` and `fp_ln(0)` will error. This is handled by the `fp_ln` zero-check, but the error message (`MathOverflow`) is misleading. Not exploitable, but could cause confusing reverts for extreme edge cases.

#### LMSR saturation

If one side's quantity grows very large relative to `b`, `fp_exp` will hit its `40 * PRECISION` cap and error. This means there's a natural ceiling on how much can be bought on one side — acceptable behavior, but should be documented and tested to know the limits.

#### Implied price exceeds 1.0

`exp_yes * PRECISION_U128 / sum` could theoretically produce a value slightly above `PRECISION` (1e9) due to rounding, violating the [0, 1] price invariant.

**Fix:** Add a `min(price, PRECISION)` clamp.

---

### Frontend

| Severity | Issue | Location |
|---|---|---|
| **MEDIUM** | No transaction simulation before submission | `app/market/[id]/page.tsx` |
| **MEDIUM** | Weak email validation on waitlist API | waitlist endpoint |
| **MEDIUM** | Unvalidated RPC responses — `decodeBase64()` has no try-catch around `atob` | `lib/mentionMarket.ts` |
| **LOW** | Balance polling instead of WebSocket subscriptions | `components/Header.tsx` |

---

## Scalability Issues

| Issue | Impact | Fix |
|---|---|---|
| `fetchAllMarkets()` loads everything into memory | RPC bottleneck at 100+ markets | Server-side indexing (see [Indexer Plan](INDEXER_PLAN.md)) |
| Trade history fetches up to 200 signatures sequentially | O(n) on network history, hits RPC rate limits | Indexer |
| No global state management / duplicate fetching | Redundant RPC calls per page load (Header + Profile both fetch escrow) | React Query / SWR cache layer |
| Cost basis recalculated on every render | Expensive client-side with 1000+ trades | Server-side computation (see [Scalability Roadmap](scalability.md)) |

---

## Dead Code

### Pages to delete

| Page | Lines | Reason |
|---|---|---|
| `app/market/trump-speech-normal/page.tsx` | ~430 | Hardcoded demo with fake data, not linked from anywhere |
| `app/market/trump-speech-pro/page.tsx` | ~430 | Same — hardcoded demo, orphaned |
| `app/archive/market/[id]/page.tsx` | ~400 | Old architecture leftover with hardcoded data |
| `app/event/[id]/pro/page.tsx` | 23 | Redirect stub — pro mode is now integrated in main event page |

~1,280 lines of dead page code.

### Components to delete

| Component | Lines | Reason |
|---|---|---|
| `components/OrderBook.tsx` | 67 | Never imported anywhere |
| `components/QuickBuy.tsx` | 113 | Never imported anywhere |
| `components/WordList.tsx` | 54 | Never imported anywhere |
| `components/Ticker.tsx` | 22 | Never imported, contains placeholder text |
| `components/ResolveRules.tsx` | 62 | Never imported anywhere |

~318 lines of dead component code.

### Other cleanup

- `lib/rich-snippets.ts` — 5 of 7 exports (`productSchema`, `softwareAppSchema`, `articleSchema`, `howToSchema`, `breadcrumbSchema`) are never imported. Only `organizationSchema` and `faqSchema` are used.
- `public/.DS_Store` — macOS artifact, should be gitignored.

---

## Verified False Positives

The following were flagged by the audit tooling but verified as non-issues:

- **`pause_market.rs` missing authority constraint** — False positive. The constraint exists at line 11: `constraint = word_market.authority == authority.key()`.
- **`init_if_needed` reinit risk on escrow** — Anchor's `init_if_needed` does not reinitialize existing accounts. If the account already exists with data, it just deserializes. The escrow PDA derivation prevents anyone else from recreating a closed account.

---

## Summary by Priority

| Priority | Item | Type |
|---|---|---|
| **HIGH** | `settle_match` unchecked backend signer (old CLOB) | Contract security |
| **HIGH** | `u64 as i64` silent overflow in math | Contract security |
| **HIGH** | No fee withdrawal instruction | Missing feature |
| **MEDIUM** | `implied_price` can exceed 1.0 | Contract math |
| **MEDIUM** | No tx simulation before submit | Frontend UX |
| **MEDIUM** | Weak waitlist email validation | Frontend security |
| **CLEANUP** | 4 dead pages (~1,280 lines) | Dead code |
| **CLEANUP** | 5 dead components (~318 lines) | Dead code |
| **CLEANUP** | 5 unused SEO schema exports | Dead code |
| **SCALABILITY** | No indexer / fetches all markets | Architecture |
| **SCALABILITY** | No shared cache layer (SWR/RQ) | Architecture |

Since the old `mention-market` CLOB contract is being replaced, the main action items are the **fee withdrawal instruction**, the **i64 cast fix**, and the **frontend dead code cleanup** (~1,600 lines removable).
