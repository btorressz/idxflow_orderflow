# idxflow_orderflow

# IDXFLOW – Orderflow Incentive Token

A **Solana-based smart contract system** that incentivizes high-frequency trading through volume-based token rewards and staking-tier fee discounts. This project was developed in Solana Playground IDE 

**devnet**:(https://explorer.solana.com/address/BH4AjERjFgoXUqJFN8GkuF1rgWQzidBwBkiibsHHpVJR?cluster=devnet)

---

## 🧠 Overview

**IDXFLOW** creates a sophisticated orderflow incentive mechanism that:

- ✅ Tracks **basket swap volume** per wallet address  
- ✅ Distributes **$IDXFLOW tokens** based on trading volume contribution  
- ✅ Provides **fee discounts** through a **multi-tier staking system**  
- ✅ Operates on **epoch-based reward cycles**

---

## 🔑 Key Features

### 📊 Volume Tracking

- Real-time tracking of **swap volume** per user  
- **Epoch-based volume reset** for fair reward distribution  
- **Configurable minimum volume thresholds** for reward eligibility  

---

### 🎯 Token Distribution

- **Automated reward calculation** based on trading volume  
- **Dynamic reward rates** (tokens per unit of volume)  
- **Staking multipliers** boost rewards for higher tiers  

---

### 🏆 Staking Tiers & Fee Discounts

| Tier       | Staking Requirement     | Fee Discount | Reward Multiplier |
|------------|-------------------------|--------------|-------------------|
| 🥉 Bronze    | 0 – 999 tokens           | 0%           | 1.0x              |
| 🥈 Silver    | 1K – 9.999K tokens       | 10%          | 1.25x             |
| 🥇 Gold      | 10K – 49.999K tokens     | 25%          | 1.5x              |
| 💎 Platinum  | 50K – 99.999K tokens     | 50%          | 2.0x              |
| 💠 Diamond   | 100K+ tokens             | 75%          | 3.0x              |

---

### ⏰ Epoch System

- Fully **configurable epoch duration** (default: 24 hours)  
- **Automatic epoch transitions**  
- **Per-epoch reward claiming** with built-in anti-double-claim protection  

---

## 🔧 Core Functions

IDXFLOW consists of two main categories of functions: **Administrative** and **User-facing**.

---

### 🛠️ Administrative Functions

These functions can only be called by the contract’s authorized admin (typically the protocol owner or governance).

#### `initialize`

This function sets up the global state for the contract. It configures the reward rate (tokens distributed per unit of trading volume), the duration of each reward epoch (measured in seconds), and the minimum volume a user must trade within an epoch to be eligible for rewards.

#### `update_reward_rate`

Allows the admin to update the reward rate. This gives the protocol flexibility to adjust token emissions as needed based on market dynamics or governance decisions.

---

