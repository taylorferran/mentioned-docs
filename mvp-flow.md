# MVP Flow

End-to-end flow for the Mentioned MVP.

---

## 1. Market creation

<div class="flow-step">
  <div class="flow-section-label">Admin</div>
  <strong>Create market</strong> — Choose a market name, set of words (up to 8), resolution deadline, and LMSR parameters.
</div>
<div class="flow-step">
  <div class="flow-section-label">On-chain</div>
  <strong>Save to contract</strong> — A single MarketAccount is created with embedded word states, YES/NO mints, and a SOL vault.
</div>

---

## 2. Liquidity provision

<div class="flow-step">
  <div class="flow-section-label">LP</div>
  <strong>Deposit liquidity</strong> — Liquidity providers deposit SOL into the market vault, receiving LP shares. This deepens the market and tightens spreads.
</div>

---

## 3. User onboarding

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Connect wallet</strong> — User connects their Solana wallet.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Deposit SOL</strong> — User deposits SOL into their <a href="#/escrow-pda">Mentioned escrow account</a>.
</div>

---

## 4. Trading

<div class="flow-step">
  <div class="flow-section-label">App</div>
  <strong>Load market</strong> — Fetch the MarketAccount from the contract, displaying words with LMSR-derived prices.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Select word</strong> — User picks a word to trade on.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Buy YES or NO</strong> — User buys tokens via the AMM. Price is calculated by LMSR based on current token quantities. SOL is deducted from escrow, tokens are minted.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Sell (optional)</strong> — User can sell tokens back to the AMM before resolution to lock in profit or cut losses.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Receive shares</strong> — YES/NO tokens land in the user's token account. SOL is drawn from their escrow balance.
</div>

---

## 5. Resolution

<div class="flow-step">
  <div class="flow-section-label">Resolver</div>
  <strong>Resolve words</strong> — Each word is resolved individually as mentioned (true) or not mentioned (false). Market becomes Resolved when all words have outcomes.
</div>

---

## 6. Settlement

<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Redeem winnings</strong> — Users holding winning tokens redeem them for SOL (1 token = 1 SOL). Payout goes to escrow.
</div>
<div class="flow-step">
  <div class="flow-section-label">User</div>
  <strong>Withdraw SOL</strong> — User withdraws SOL from their escrow account.
</div>
<div class="flow-step">
  <div class="flow-section-label">LP</div>
  <strong>Withdraw liquidity</strong> — LPs burn their shares and withdraw proportional SOL from the vault.
</div>
