# MVP Flow

End-to-end user journey on Mentioned.

---

## 1. Connect wallet

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Connect</strong> — Phantom or Privy wallet. Auto-reconnect restores the session on return visits.
</div>

---

## 2. Set up profile

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Set username & emoji</strong> — Optional display name (3–20 chars, unique) and profile emoji. Unlocks achievements and points (75 + 50 pts).
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Link Discord</strong> — Required to earn any points. One Discord per wallet. Assigns verified role in the Mentioned Discord.
</div>

---

## 3. Trade (Paid Markets)

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Browse events</strong> — Live and upcoming events on <code>/markets</code>. Click an event to open the trading interface.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Place order</strong> — Select YES or NO, enter amount. Sent to Jupiter via API proxy.
</div>
<div class="flow-step">
  <div class="flow-section-label">Wallet</div>
  <strong>Sign transaction</strong> — Phantom or Privy signs the unsigned tx returned by Jupiter.
</div>
<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Earn points</strong> — 10 pts per trade (daily cap 20), 100 pts for first ever trade. Discord link required.
</div>

---

## 4. Trade (Free Markets)

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Browse free markets</strong> — Virtual LMSR markets with play tokens. No real money.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Buy/sell shares</strong> — LMSR pricing from the virtual pool. Deducted from play token balance.
</div>
<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Earn points on win</strong> — After resolution, 50% of net token profit converted to points.
</div>

---

## 5. Settlement (Paid Markets)

<div class="flow-step">
  <div class="flow-section-label">Market</div>
  <strong>Event settles</strong> — Polymarket resolves based on real-world outcome.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Claim winnings</strong> — Green Claim button on winning positions. Sign payout tx to receive funds.
</div>
<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Earn points</strong> — 50 pts for each winning claim.
</div>

---

## 6. Compete

<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Points leaderboard</strong> — Weekly rankings by total points. Resets Monday UTC. Discord-linked users only.
</div>
<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Trading leaderboard</strong> — Weekly P&L, volume, and win rate rankings.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Chat</strong> — Global and per-event chat. Earns 2 pts per message (daily cap 10).
</div>
