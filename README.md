# idxflow_orderflow

# IDXFLOW – Orderflow Incentive Token

A **Solana-based smart contract system** that incentivizes high-frequency trading through volume-based token rewards and staking-tier fee discounts.

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
