# MVP Flow

End-to-end flow for the Mentioned MVP.

---

## 1. Market creation

<div class="flow-step">
  <div class="flow-section-label">Admin</div>
  <strong>Create market</strong> — Choose a market name and the set of words to trade on.
</div>
<div class="flow-step">
  <div class="flow-section-label">On-chain</div>
  <strong>Save to contract</strong> — Market name and word list are written to the market program on Solana.
</div>

---

## 2. User onboarding

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Connect wallet</strong> — User connects their Solana wallet.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Deposit SOL</strong> — User deposits SOL into their <a href="#/escrow-pda">Mentioned escrow account</a>.
</div>

---

## 3. Trading

<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Load markets</strong> — Fetch all available markets from the contract.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Select market</strong> — User picks a market to trade in.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Place order</strong> — User chooses to market buy YES or NO with SOL. For example, pay 0.5 SOL on YES at $0.50.
</div>
<div class="flow-step">
  <div class="flow-section-label">On-chain</div>
  <strong>Match trade</strong> — The buy is matched with a counterparty on the opposite side.
</div>
<div class="flow-step">
  <div class="flow-section-label">On-chain</div>
  <strong>Mint tokens</strong> — YES and NO tokens are minted for both the buyer and the matched counterparty.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Receive shares</strong> — YES/NO shares land in the user's account. SOL is drawn from their escrow balance.
</div>

---

## 4. Resolution

<div class="flow-step">
  <div class="flow-section-label">Admin</div>
  <strong>Resolve market</strong> — Each word in the market is resolved as YES or NO.
</div>

---

## 5. Settlement

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Claim winnings</strong> — Users holding winning YES tokens redeem them for SOL.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Withdraw SOL</strong> — User withdraws remaining SOL from their escrow account.
</div>
