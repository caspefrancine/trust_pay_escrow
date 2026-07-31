# 🛡️ Frontline Ledger (Level 4 Production MVP)

> A Decentralized Document & Credential Authentication dApp on the Stellar Testnet using Soroban Smart Contracts and Local SHA-256 Cryptographic Hashing.

---

## 📌 Problem & Solution

* **Problem:** Educational institutions and frontline public offices face widespread document forgery, diploma fraud, and slow manual clearance verification processes that require back-and-forth email/paper validation.
* **Solution:** **Frontline Ledger** enables institutions to issue tamper-proof cryptographic document hashes directly onto Soroban smart contracts. Citizens and employers can upload any document to locally compute its SHA-256 hash and verify its authenticity on-chain in seconds without exposing confidential file contents.

---

## 🚀 Vision & Purpose

Designed for security, speed, and privacy, Frontline Ledger digitalizes credential authentication across academic and municipal frontline systems in the Philippines and Southeast Asia—eliminating paper fraud and streamlining verification using zero-trust blockchain technology.

---

## 🔗 Important Links

* **Live Production MVP:** [https://caspefrancine.github.io/trust_pay_escrow/](https://caspefrancine.github.io/trust_pay_escrow/)
* **Demo Video Presentation (1-2 Min):** [INSERT_YOUR_LOOM_OR_DRIVE_VIDEO_LINK_HERE]
* **Stellar Testnet Explorer:** [`https://stellar.expert/explorer/testnet/tx/a78f3c92109ba4d7e822019a32cba928182903e12918820f`](https://stellar.expert/explorer/testnet/tx/a78f3c92109ba4d7e822019a32cba928182903e12918820f)

---

## 🛠️ Level 4 Features & Production Architecture

* **Soroban Smart Contract Backend:** Rust-based contract managing Role-Based Access Control (RBAC) for issuers, timestamped SHA-256 document registration, and revocation states.
* **Local SHA-256 Browser Hashing:** Client-side Web Crypto API integration that calculates document hashes locally inside the browser for maximum data privacy.
* **Multi-Wallet Integration:** Powered by `@creit.tech/stellar-wallets-kit` supporting Freighter, Albedo, and xBull wallets.
* **Mobile Responsive UI:** Fluid, accessible interface optimized for desktop, tablet, and mobile devices.
* **Analytics & Monitoring Setup:** Integrated Google Analytics script for user traffic and event tracking.
* **SaaS Feedback Portal:** Integrated floating feedback trigger connecting real-world users to continuous product validation.

---

## 👥 User Onboarding & Validation Summary

We successfully onboarded **15 real users** (students, faculty/admins, peer developers, and verifiers) to test the Frontline Ledger MVP on the Stellar Testnet.

### 📈 Key Feedback Metrics:
* **User Diversity:** 73.3% Students, 13.3% Verifiers, along with Peer Developers and Faculty/Admins.
* **System Usability:** 100% of testers gave a **5/5 rating** for ease of wallet connection and interface navigation.
* **Hashing Performance:** 100% confirmed instant local document verification with zero latency.

---

## 📸 Level 4 Required Proof & Screenshots

### 1. Product UI (Connected Wallet)
![Product UI](connected%20wallet%20with%20feedback.jpg)

### 2. Mobile Responsive Design
![Mobile Responsive UI](mobile%20view%20with%20feedback.jpg)

### 3. Monitoring & Analytics Integration
![Analytics Setup](analytics%20code%20setup.jpg)

### 4. 10+ Real User Feedback & Onboarding Proof
![User Feedback Summary](users%20feedback.jpg)

---

## 📜 Deployed Smart Contract Info

* **Network:** Stellar Testnet
* **Smart Contract:** `FrontlineLedger` (Soroban SDK `#![no_std]`)
* **Functions:** `initialize`, `add_issuer`, `issue_doc`, `verify_doc`, `revoke_doc`

---

## 💻 Setup & Local Development

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/caspefrancine/trust_pay_escrow.git](https://github.com/caspefrancine/trust_pay_escrow.git)
   cd trust_pay_escrow


## 📽️ Product Walkthrough & Demo Video

Watch a short 2-minute walkthrough showcasing **Frontline Ledger** in action—covering Freighter wallet integration, instant local SHA-256 document hashing, Soroban smart contract verification, and user feedback:

▶️ **[Click Here to Watch the Frontline Ledger Level 4 MVP Demo]((https://drive.google.com/file/d/1CQv6W3VTyuA_V5KkK75By2t8fh6kFRNm/view))**
