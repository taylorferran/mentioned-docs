# resolve_word

Resolve a single word within a market as mentioned (`true`) or not mentioned (`false`). When all words in a market are resolved, the market status automatically transitions to `Resolved`.

**Caller:** Resolver

## Parameters

| Name | Type | Description |
|---|---|---|
| word_index | u8 | Which word to resolve (0-7) |
| outcome | bool | `true` = word was mentioned, `false` = not mentioned |

## Accounts

| Account | Type | Description |
|---|---|---|
| resolver | Signer | Must match `market.resolver` |
| market | Account, mut | The market |

## Logic

1. Validate caller is the designated resolver
2. Validate market is not already fully `Resolved`
3. Validate word is not already resolved
4. Set `word.outcome = Some(outcome)`
5. Check if all words are now resolved
6. If yes → set `market.status = Resolved` and record `resolved_at` timestamp
7. Emit `ResolutionEvent`

## Auto-resolution

The market does not resolve in one call. Each word is resolved individually. The market transitions to `Resolved` only when **every** word has an outcome set. This allows partial resolution as results come in.

## Errors

| Error | Condition |
|---|---|
| UnauthorizedResolver | Signer is not the market's designated resolver |
| MarketAlreadyResolved | Market is already fully resolved |
| InvalidWordIndex | Word index out of range |
| WordAlreadyResolved | This specific word already has an outcome |

## Events

```
ResolutionEvent {
  market_id, word_index, outcome, resolver, timestamp
}
```
