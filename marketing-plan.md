# Marketing Plan — X & LinkedIn

## Strategy

Mentioned is a prediction market on Solana where users trade YES/NO tokens on whether specific words will be mentioned in defined contexts. Up to 8 words per market, prices set by an on-chain AMM (LMSR), winners redeem tokens 1:1 for SOL.

The audience splits into two camps: crypto-native traders who already get on-chain mechanics, and prediction market users coming from Polymarket who might be newer to Solana.

Tone: sharp, slightly irreverent, confident but not tryhard. "The smart friend who's early to things" — not "corporate brand trying to be cool."

**Cadence:**
- X: 5-7 posts per week (market callouts, explainers, engagement bait, commentary)
- LinkedIn: 2-3 posts per week (industry, tech, founder perspective)

---

## Content Pillars

| Pillar | Share | Focus |
|---|---|---|
| Market callouts | 40% | Highlight active/upcoming markets, create urgency |
| Education | 20% | How LMSR works, the trading flow, what resolution looks like |
| Takes & commentary | 20% | React to real events, tie back to "you could have traded this" |
| Community & social proof | 10% | Wins, volume milestones, funny trades |
| Builder / industry | 10% | Why Solana, why LMSR, what's next (skews LinkedIn) |

---

## X Draft Posts

### Market Callouts

**Post 1:**
> New market just dropped.
>
> 8 words. You pick which ones get said. Buy YES or NO on each.
>
> Prices move with demand — on-chain AMM sets them automatically. When it resolves, winners redeem for SOL.
>
> [link]

**Post 2:**
> Everyone has opinions. Few people put SOL behind them.
>
> Mentioned lets you trade on whether specific words get mentioned in a given context — speeches, earnings calls, podcasts, whatever.
>
> Right call = SOL. Wrong call = someone else's SOL.

**Post 3:**
> The market is saying "economy" has a 74% chance of being mentioned in the State of the Union.
>
> You agree? Buy YES.
> You think it's overblown? Buy NO at $0.26 and cash out at $1 when it doesn't happen.
>
> That's the whole game.

**Post 4:**
> Will Apple say "AI" at WWDC? Will the CEO mention "layoffs" on the Q3 call? Will MrBeast say "subscribe" in his next video?
>
> If there's an audience and there are words, there's a Mentioned market.

---

### Education

**Post 5:**
> How pricing works on Mentioned:
>
> We use LMSR on-chain. It's an automated market maker — but for predictions, not token swaps.
>
> More people buying YES → price goes up. More buying NO → price drops. The AMM adjusts automatically based on token quantities.
>
> No order book. No matching. Just connect wallet and trade.

**Post 6:**
> How a trade works on Mentioned:
>
> 1. Connect your Solana wallet
> 2. Deposit SOL into your escrow
> 3. Pick a word in an active market
> 4. Buy YES or NO tokens (price set by AMM)
> 5. Sell anytime before resolution to lock in profit
> 6. Market resolves — winning tokens redeem 1:1 for SOL
>
> All on Solana. All on-chain. Tokens show up in your wallet as "Economy YES" or "Bitcoin NO."

**Post 7:**
> "Wait so how does the pricing actually work?"
>
> Each word starts at $0.50 (equal chance). When you buy YES, the price ticks up. When someone else buys NO, it ticks down.
>
> The formula: p = e^(q/b) / (e^(q_yes/b) + e^(q_no/b))
>
> You don't need to know the math. Just know: cheap = the crowd disagrees with you. That's where the edge is.

**Post 8:**
> Your YES/NO tokens aren't just numbers on a screen.
>
> They're real SPL tokens on Solana with Metaplex metadata. Open your Phantom wallet and you'll see "Bitcoin YES" or "Economy NO" with proper names and symbols.
>
> Everything on-chain. Everything verifiable.

---

### Takes & Commentary

**Post 9:**
> Every press conference, every earnings call, every speech is just a collection of words someone chose to say.
>
> Now you can trade on which words those will be.

**Post 10:**
> Prediction markets are good at forecasting outcomes.
>
> But outcomes are made of language. We're going one layer deeper.

**Post 11:**
> When "tariff" is trading at 90% YES before a trade speech, the crowd is telling you something.
>
> When it drops to 40% after the first five minutes, that's a signal too.
>
> Word-level prediction markets reveal expectations in real time.

**Post 12:**
> The World Cup is coming.
>
> Will the commentator say "golazo" during El Clasico? Will "upset" get mentioned at the final?
>
> Sports commentary is the most untapped prediction market vertical. And we're building it.

---

### Community & Engagement

**Post 13:**
> What word do you think is most overpriced right now?
>
> Quote this with your pick.

**Post 14:**
> Somebody just loaded up on NO tokens for a word the rest of the market is convinced will be mentioned.
>
> Either they know something or they're about to learn an expensive lesson.

**Post 15:**
> Still early. The people trading on Mentioned right now are going to look back on these markets the way early Polymarket users look back on 2020.
>
> Except we're on Solana and settlement is instant.

---

### Memes (X only)

**Post 16:**
> "What do you do for work?"
>
> I bet on whether people will say specific words
>
> "..."
>
> On the blockchain
>
> "......"

**Post 17:**
> Prediction markets but make it linguistics

**Post 18:**
> POV: you bought YES on a word at $0.15 and it just got mentioned live on air
>
> 1 token = 1 SOL. Do the math.

---

## LinkedIn Draft Posts

### Post L1: Founder Intro

> Most prediction markets ask you to bet on outcomes. Will X happen? Will Y win?
>
> We asked a different question: what if you could trade on the language itself?
>
> Mentioned is a prediction market on Solana where users trade on whether specific words will be mentioned in defined contexts. Each market has up to 8 words. For each one, you buy YES or NO tokens — real SPL tokens that show up in your wallet. Prices are set by an on-chain AMM using LMSR, and when the market resolves, winners redeem 1:1 for SOL.
>
> Why words? Because language is the most granular signal of intent. Before any policy, decision, or announcement becomes an outcome, it starts as a choice of words. We think there's real alpha in that layer.
>
> The categories are wide open: earnings calls ("layoffs"), political speeches ("economy"), sports commentary ("golazo"), podcasts ("consciousness"), product launches ("AI"). If there's an audience and words are being spoken, there's a market.
>
> Live on Solana devnet now, mainnet coming in March.

### Post L2: Industry Commentary

> Prediction markets had a breakout year. Polymarket proved the model at scale. Kalshi got regulatory clarity. The space went from niche to mainstream.
>
> But most prediction markets still operate at a high level of abstraction. Binary outcomes, resolved weeks or months later.
>
> We think there's room for something more granular and more frequent. At Mentioned, markets resolve around specific words being mentioned in specific contexts — speeches, earnings calls, live events. It's faster, more precise, and opens up a category of events that didn't have markets before.
>
> The architecture is fully on-chain: LMSR pricing in the smart contract, YES/NO tokens as real SPL tokens with wallet metadata, a dedicated resolver role for transparent outcomes, and trade events indexed via Helius webhooks into Postgres for real-time charts and history.
>
> MVP on Solana devnet now. Mainnet launch planned for late March.

### Post L3: Why Solana

> When we chose where to build Mentioned, it came down to a few things:
>
> Speed matters. Our markets need fast settlement and cheap transactions because users make multiple trades per market, sometimes reacting in real time to live events. Solana handles that without users thinking about gas.
>
> On-chain AMM. We run LMSR pricing directly in the smart contract. Every price update, every token mint, every trade is verifiable. The YES/NO tokens have Metaplex metadata — they show up in Phantom as "Bitcoin YES" or "Economy NO" with proper names and symbols. No off-chain order books, no trust assumptions.
>
> Trade events are indexed in real-time via Helius webhooks into Postgres, so charts and trade history load instantly without hammering RPC nodes. The infrastructure is production-grade from day one.
>
> We're not building a DeFi protocol that happens to do predictions. We're building a prediction market that happens to be on-chain. The distinction matters.

---

## Engagement Playbook

- **React to news** with "there should be a Mentioned market for this." Do it often enough and it becomes a meme.
- **Quote tweet** prediction market takes and add the Mentioned angle.
- **Keep it short.** Crypto twitter respects brevity. If a post needs more than 4 sentences, it better be really good.
- **Pin a thread** explaining how Mentioned works in plain language. Update it monthly.
- **Run polls** asking what markets people want to see. Then actually create them. Closed loop = engagement.
- **Post market results** after resolution. "The crowd said 74% YES on 'economy.' Final result: mentioned at 4:32. Winners paid out in SOL." This is the social proof loop.
- On LinkedIn, **lean into "why this matters"** more than mechanics. Decision-makers care about the signal, not the AMM curve.

---

## First Two Weeks Schedule

| Day | X | LinkedIn |
|---|---|---|
| Mon | Market callout (Post 1) | Founder intro (L1) |
| Tue | Education — how a trade works (Post 6) | — |
| Wed | Take (Post 9) | — |
| Thu | Engagement — overpriced word? (Post 13) | Industry commentary (L2) |
| Fri | Meme (Post 16) | — |
| Sat | Community (Post 14) | — |
| Sun | Take (Post 11) | — |
| Mon | Market callout (Post 3) | Why Solana (L3) |
| Tue | Education — pricing (Post 5) | — |
| Wed | Meme (Post 17) | — |
| Thu | Take — World Cup (Post 12) | Adapt from week's best X content |
| Fri | Community (Post 15) | — |
| Sat | Education — tokens in wallet (Post 8) | — |
| Sun | Engagement — poll on next market | — |

---

## Key Facts for Content

Reference sheet for writing future posts:

| Fact | Detail |
|---|---|
| Chain | Solana |
| Pricing | LMSR (Logarithmic Market Scoring Rule) — on-chain AMM |
| Tokens | YES/NO SPL tokens with Metaplex metadata (e.g. "Bitcoin YES" / BITC-Y) |
| Token decimals | 9 |
| Redemption | 1 winning token = 1 SOL |
| Words per market | Up to 8 |
| Resolution | Per-word, by a designated resolver |
| Trading flow | Deposit SOL → escrow → buy/sell YES/NO → resolution → redeem → withdraw |
| Indexer | Helius webhooks → Postgres → REST API for charts and history |
| Market categories | Entertainment, politics/business, sports, written content, timing markets |
| Current status | Devnet (mainnet targeting late March) |
| Competitors / context | Polymarket ($1 = 1 share), Kalshi. We're SOL-denominated and word-level granular |
