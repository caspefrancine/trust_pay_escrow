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


# 🛡️ TrustPay Escrow (Level 2 dApp)

> A Web3 micro-payroll and escrow dApp protecting freelancers from non-payment on the Stellar Testnet using Soroban Smart Contracts.

---

## 📌 Problem & Solution

* **Problem:** A freelance UI designer in Manila loses $400 in uncompensated work after delivering Figma prototypes to an overseas client who abruptly cuts communication without paying.
* **Solution:** The dApp locks the client's funds into a Stellar Testnet escrow before design work begins and automatically releases the XLM payout to the freelancer once the completed task link is approved.

---

## 🛠️ Level 2 Features & Requirements Met

* **Contract Deployed on Testnet:** Deployed Soroban smart contract managing escrow creation, submission, and release.
* **Multi-Wallet Integration:** Integrated `@creit.tech/stellar-wallets-kit` supporting Freighter, Albedo, and xBull wallets.
* **3 Error Types Handled:** Explicit UI feedback for:
  1. *Wallet Not Found / Uninstalled*
  2. *User Rejected / Declined Transaction*
  3. *Insufficient Balance / Account Underfunded*
* **Real-time Status Tracking:** Dynamic transaction state updates (`Pending`, `Confirmed`, `Failed`).

---
## 📸 Wallet Options Screenshot
![Multi-Wallet Selection](wallet-modal.jpg)

---

## 📜 Deployed Contract & Explorer Verification

* **Deployed Contract Address:** `CAGLK3ZC7Y4XEDO046DT5FZ6DK4VY5BGFH3TZKDAJ4ZN5TPZWQW6SIVA`
* **Verified Contract Call Transaction Hash:** [`1fa8a866a4bdf718343ff243e4899b333e3444e347f8e7ff7c995dd4`](https://stellar.expert/explorer/testnet/tx/1fa8a866a4bdf718343ff243e4899b333e3444e347f8e7ff7c995dd4)

---

## 🚀 Vision & Purpose
TrustPay aims to eliminate payment default risks and predatory 20% commission rates charged by web2 freelance platforms across Southeast Asia by using zero-trust Soroban smart contract escrows.

---

## 💻 Setup Instructions

1. Clone the public repository:
   ```bash
   git clone [https://github.com/caspefrancine/trust_pay_escrow.git](https://github.com/caspefrancine/trust_pay_escrow.git)

# 🛡️ TrustPay Escrow (Level 3 dApp)

> A Web3 micro-payroll and escrow dApp empowering freelancers and UI/UX designers to securely digitalize their workflow on the Stellar Testnet using Soroban Smart Contracts.

---

## 📌 Problem & Solution

* **Problem:** Freelance creatives often lose out on uncompensated work after delivering prototypes to clients who abruptly cut communication without paying.
* **Solution:** TrustPay locks the client's funds into a Stellar Testnet escrow before work begins and automatically releases the XLM payout once the completed task is approved, ensuring a zero-trust, secure transaction.

---

## 🚀 Vision & Purpose
Designed with a focus on intuitive UI/UX, TrustPay aims to eliminate payment default risks and predatory 20% commission rates charged by traditional web2 platforms. It provides a secure, decentralized way for freelancers in the Philippines and across Southeast Asia to digitalize their payment processes.

---

## 🔗 Important Links
* **Live Demo:** [Insert your GitHub Pages or Vercel Link Here]
* **Video Presentation (1-2 Min):** [Insert your Loom or Google Drive Video Link Here]

---

## 🛠️ Level 3 Features & Architecture

* **Advanced Smart Contracts:** Rust-based Soroban smart contract managing state transitions (Funded, Submitted, Completed) and secure escrow logic.
* **Automated CI/CD Pipeline:** GitHub Actions workflow configured for automated Rust toolchain installation, caching, and testing on every push to the main branch.
* **Robust Test Suite:** Comprehensive unit testing verifying initialization, deposit locks, payout releases, and unauthorized access edge cases.
* **Mobile Responsive Frontend:** Fluid, accessible UI ensuring seamless operation and wallet connection across both desktop and mobile devices.
* **Multi-Wallet Integration:** Support for Freighter, Albedo, and xBull via `@creit.tech/stellar-wallets-kit`.
* **State & Error Handling:** Explicit UI state management for rejected transactions, missing wallets, and insufficient funds.

---

## 📜 Smart Contract Deployment

* **Network:** Stellar Testnet
* **Deployed Contract Address:** `CAGLK3ZC7Y4XEDOO46DT5FZ6DK4VY5BGFH3TZKDAJ4ZN5TPZWQW6SIVA`
* **Verified Transaction Hash:** [`1fa0a066a44bdf718343ff243e4899b333e3444e347f8e7ff7c995dd46f342a6`](https://stellar.expert/explorer/testnet/tx/1fa0a066a44bdf718343ff243e4899b333e3444e347f8e7ff7c995dd46f342a6)

---

## 📸 Project Verification Screenshots

### 1. Mobile Responsive UI
![Mobile Responsive UI](insert-your-mobile-ui-screenshot-filename-here.png)

### 2. CI/CD Pipeline Success
![CI/CD Pipeline Running](cicd-success.png)

### 3. Automated Contract Tests (Passing)
![Passing Tests Output](test-output.png)

---

## 💻 Setup & Local Development

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/caspefrancine/trust_pay_escrow.git](https://github.com/caspefrancine/trust_pay_escrow.git)
   cd trust_pay_escrow
