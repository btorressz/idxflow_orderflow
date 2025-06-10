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

### 🙋 User Functions

These functions are accessible by users participating in the protocol, such as traders or liquidity providers.

#### `create_user_account`

Sets up a user-specific on-chain account to track their trading volume, staking status, and reward eligibility.

#### `record_swap_volume`

This function is called after a trade is executed. It logs the amount of volume the user generated so it can be used for calculating rewards.

#### `stake_tokens`

Users can stake $IDXFLOW tokens into the protocol. Doing so unlocks higher staking tiers, which grant increased fee discounts and reward multipliers.

#### `unstake_tokens`

Users can withdraw (unstake) their tokens. Doing so may lower their fee tier, depending on how much they withdraw.

#### `claim_rewards`

After an epoch ends, eligible users can call this function to claim their earned rewards. The system ensures rewards are only claimed once per epoch.

#### `get_fee_discount`

Returns the user’s current fee discount percentage based on their staking tier. Useful for DEXs or aggregators applying discounted trading fees.

---

## 🧱 Account Structure

The contract uses two main account types to store global and user-specific data.

### `GlobalState`

Holds configuration values and protocol-wide metrics, including:

- Admin authority  
- Reward rate (tokens per volume unit)  
- Epoch duration and tracking  
- Minimum volume threshold  
- Total rewards distributed  

### `UserAccount`

Tracks individual user metrics, including:

- Wallet address  
- Total and epoch trading volume  
- Amount of tokens staked  
- Current fee tier  
- Last epoch when rewards were claimed  

---

## 🔐 Security Features

IDXFLOW is designed with robust security measures:

- **Program Derived Addresses (PDAs)**: Ensure secure and deterministic account derivation for both global and user state.
- **Epoch-Based Claiming**: Prevents duplicate reward claims in the same epoch.
- **Authority Checks**: Only the authorized admin can perform configuration changes.
- **Checked Arithmetic**: All reward and volume calculations use safe math to prevent overflow.
- **Minimum Volume Thresholds**: Blocks low-effort spam or volume manipulation attacks.

---

## ⚠️ Error Handling

IDXFLOW includes several error guards to prevent misuse:

- **InsufficientStakedAmount**: Thrown if a user tries to unstake more tokens than they have staked.
- **AlreadyClaimedThisEpoch**: Prevents a user from claiming rewards more than once per epoch.
- **InsufficientVolume**: Indicates the user didn’t meet the minimum required trading volume to be eligible for rewards.
- **Unauthorized**: Blocks unauthorized accounts from modifying protocol parameters.

---


