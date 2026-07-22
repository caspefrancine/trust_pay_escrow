# TrustPay Escrow

> A Web3 micro-payroll and escrow dApp protecting freelancers from non-payment on the Stellar Testnet.

---

## 📌 Problem & Solution

* **Problem:** A freelance UI designer in Manila loses $400 in uncompensated work after delivering Figma prototypes to an overseas client who abruptly cuts communication without paying.
* **Solution:** The dApp locks the client's funds into a Stellar Testnet escrow before design work begins and automatically releases the XLM payout to the freelancer once the completed task link is approved.

---

## 🛠️ Stellar Features Used
* Soroban Smart Contracts
* Native XLM Transfers / Payments
* Testnet Horizon Explorer Verification

---

## 🚀 Vision & Purpose
TrustPay aims to eliminate payment default risks and predatory 20% commission rates charged by web2 freelance platforms across Southeast Asia by using zero-trust Soroban smart contract escrows.

---

## 📋 Prerequisites
* Rust `target wasm32-unknown-unknown`
* Soroban CLI (`v20.0.0` or higher)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli

✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/1fa0a066a44bdf718343ff243e4899b333e3444e347f8e7ff7c995dd46f342a6
🔗 https://lab.stellar.org/r/testnet/contract/CAGLK3ZC7Y4XEDOO46DT5FZ6DK4VY5BGFH3TZKDAJ4ZN5TPZWQW6SIVA
✅ Deployed!
CAGLK3ZC7Y4XEDOO46DT5FZ6DK4VY5BGFH3TZKDAJ4ZN5TPZWQW6SIVA