# MVP Flow

End-to-end flow for the Mentioned MVP — a social prediction market platform built on Polymarket.

---

## 1. Connect wallet

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Connect Phantom</strong> — User connects their Solana wallet. Auto-reconnect restores the session on return visits.
</div>

---

## 2. Set up profile

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Choose username</strong> — Optional display name (3–20 chars, unique). Appears on leaderboard and in chat.
</div>

---

## 3. Browse events

<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Load events</strong> — Fetch esports events from Jupiter's Polymarket API. Displayed as Live Now and Upcoming.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Select event</strong> — Click into an event to see markets, orderbook, and trading panel.
</div>

---

## 4. Trade

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Place order</strong> — Select YES or NO, enter amount. Order sent to Jupiter via our API proxy.
</div>
<div class="flow-step">
  <div class="flow-section-label">Wallet</div>
  <strong>Sign transaction</strong> — Phantom prompts to sign the unsigned transaction returned by Jupiter.
</div>
<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Record trade</strong> — Trade saved to DB for leaderboard tracking.
</div>

---

## 5. Manage positions

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Track positions</strong> — View open positions, unrealized P&L, and open orders from the Positions page or Profile.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Close early</strong> — Sell a position before settlement to lock in profit or cut losses.
</div>

---

## 6. Settlement

<div class="flow-step">
  <div class="flow-section-label">Market</div>
  <strong>Event settles</strong> — Market resolves based on real-world outcome.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Claim winnings</strong> — Winning positions show a Claim button. Sign the payout transaction to receive funds.
</div>

---

## 7. Compete

<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Leaderboard</strong> — Weekly rankings by P&L, volume, and win rate. Resets every Monday UTC.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Chat</strong> — Real-time global chat with other traders.
</div>
