# Uniswap V2 & DeFi Study Notes

## Table of Contents
- [Core Concepts](#core-concepts)
- [Uniswap V2 Overview](#uniswap-v2-overview)
- [CPMM (Constant Product Market Maker)](#cpmm-constant-product-market-maker)
- [DeFi Concepts](#defi-concepts)
- [Impermanent Loss](#impermanent-loss)
- [Swap Math Deep Dive](#swap-math-deep-dive)
- [Advanced Topics](#advanced-topics)

---

## Core Concepts

### What is Uniswap V2?

**One-line explanation:**  
Uniswap V2 is a smart-contract-based decentralized exchange where liquidity pools set token prices using a mathematical formula instead of orderbooks.

**How it works:**
1. **Liquidity Pools:** Two tokens (e.g., ETH and USDC) are deposited into a pool
2. **Liquidity Providers (LPs):** Users deposit both tokens and earn trading fees
3. **Traders:** Swap tokens directly inside the pool
4. **Automatic Pricing:** Price adjusts automatically after every trade based on the formula

**Core principle:**  
The pool must always maintain: `x * y = k`

Where:
- `x` = amount of token0 in the pool
- `y` = amount of token1 in the pool
- `k` = constant total liquidity value

---

## CPMM (Constant Product Market Maker)

### Definition
CPMM is the mathematical engine behind Uniswap V2. The pool must always keep the product of both token balances constant, which makes the price move automatically.

### The Formula
```
x * y = k (constant)
```

### How CPMM Works

Think of it like a seesaw holding a constant product:
- If a trader wants token X, they must add token Y
- Taking one token forces the other token's amount to rise so the product stays constant
- This creates **slippage** and automatic price movement

### Example

**Initial Pool State:**
```
x = 10 ETH
y = 20,000 USDC
k = 200,000
```

**If someone buys ETH:**
- ETH amount decreases (x goes down)
- To keep `xy = k` constant, USDC must increase
- So the trader must pay more USDC for each ETH
- This is why bigger trades move the price more

### Price Formula in CPMM

**Spot price** of token0 (ETH) in terms of token1 (USDC):
```
price = y / x
```

**Actual swap calculation:**
```
(x + Δx) * (y - Δy) = k
```
The smart contract solves for `Δy` (how many tokens you get out).

---

## DeFi Concepts

### Automated Market Makers (AMMs)
- Pool-based trading system
- No orderbook, no buyer/seller matching
- Price determined by mathematical formula
- Liquidity provided by users (LPs)

### Liquidity Pools
- Smart contracts holding two tokens
- Anyone can deposit to become an LP
- LPs earn a share of trading fees
- Price adjusts automatically based on trades

### Yield Farming
**Definition:** Users deposit tokens into DeFi protocols to earn extra rewards (interest or fees).

**One-line:** You deposit tokens somewhere and earn rewards over time.

**Example:** Put ETH–USDC LP tokens into a special contract → earn additional rewards

### Liquidity Mining
**Definition:** A type of yield farming where protocols give their own token as a reward to liquidity providers.

**One-line:** You provide liquidity and the protocol pays you with its native token.

**Example:** Provide liquidity → receive UNI tokens as reward

### Summary Table

| Concept | Meaning |
|---------|---------|
| AMM | Pool that sets prices using formulas (no orderbook) |
| Liquidity Pool | You deposit two tokens into AMM |
| Yield Farming | You deposit tokens to earn rewards |
| Liquidity Mining | Rewards paid in the protocol's own token |

---

## Impermanent Loss

### Simple Explanation

**One-line:**  
Impermanent loss happens because arbitrage traders remove the expensive token from the pool and leave you holding more of the cheaper token.

### Real-Life Example

**You add liquidity:**
- 1 ETH
- 100 USDC
- Price: 1 ETH = 100 USDC

**Assume total pool holds (for simplicity):**
- 10 ETH
- 1,000 USDC
- You own 10% of the pool

**ETH price rises on external market:**
- ETH goes from 100 → 200 USDC on Binance/Coinbase
- But Uniswap pool still reflects old price (100)
- Arbitrage traders notice the mismatch

**Arbitrage traders buy cheap ETH from pool:**

Before:
```
10 ETH | 1,000 USDC
```

After arbitrage:
```
~7.071 ETH | ~1,414 USDC
```

**Your 10% share now contains:**
- 0.7071 ETH (was 1 ETH)
- 141.4 USDC (was 100 USDC)

**Value comparison:**

If you had just held (HODL):
```
1 ETH × 200 = 200 USDC
100 USDC = 100 USDC
Total = 300 USDC
```

In the pool after price change:
```
0.7071 ETH × 200 = 141.4 USDC
141.4 USDC = 141.4 USDC
Total = 282.8 USDC
```

**Impermanent Loss = 300 - 282.8 = 17.2 USDC (≈5.7%)**

### Why It Happens

The AMM formula `x * y = k` forces the pool to rebalance when price changes:
- Pool gives you **more of the stable/low-value asset**
- Pool takes away **the high-value asset**
- You end up with imbalanced tokens

### Why "Impermanent"?

Because it's not final until you withdraw. If prices return to the original level, the loss disappears.

### Impermanent Loss Formula

For price change by factor `R = newPrice / oldPrice`:

```
IL(R) = 1 - (2 * √R / (R + 1))
```

**Example with R = 2 (price doubles):**
```
IL = 1 - (2 * √2 / 3)
IL ≈ 0.0572 → 5.72% loss
```

---

## Swap Math Deep Dive

### Basic Setup

**Pool invariant:**
```
x * y = k (constant)
```

**Notation:**
- `x₀` = current amount of token X in pool (BEFORE swap)
- `y₀` = current amount of token Y in pool (BEFORE swap)
- `dx` = input (what trader gives to the pool)
- `dy` = output (what trader receives from the pool)

**Note:** The subscript ₀ (zero) means "initial" or "starting value" — it's just a label, NOT multiplication.

### Example Pool State

```
x₀ = 10 ETH
y₀ = 1,000 USDC
```

### The Invariant Rule

**Before trade:**
```
x₀ * y₀ = k
```

**After trade:**
```
(x₀ + dx) * (y₀ - dy) = k
```

The pool must stay on the same curve!

### Deriving the Swap Formula (No Fees)

**Step 1:** Start from invariant equality
```
(x₀ + dx) * (y₀ - dy) = x₀ * y₀
```

**Step 2:** Expand and simplify
```
x₀*y₀ + dx*y₀ - x₀*dy - dx*dy = x₀*y₀
```

Ignoring the tiny `dx*dy` term:
```
dx*y₀ - x₀*dy = 0
```

**Step 3:** Isolate `dy`
```
dx*y₀ = x₀*dy
dy = (y₀ * dx) / x₀
```

But wait, this is the simplified version. The exact formula is:

**Step 4:** Solve from the invariant directly
```
y₀ - dy = (x₀ * y₀) / (x₀ + dx)
dy = y₀ - (x₀*y₀)/(x₀+dx)
dy = y₀ * (1 - x₀/(x₀+dx))
dy = y₀ * ((x₀+dx-x₀)/(x₀+dx))
```

**Final formula (no fees):**
```
dy = (y₀ * dx) / (x₀ + dx)
```

### Concrete Example

**Pool state:**
```
x₀ = 10 ETH
y₀ = 1,000 USDC
```

**Alice swaps:**
```
dx = 1 ETH
```

**Calculate output:**
```
dy = (1,000 * 1) / (10 + 1)
dy = 1,000 / 11
dy ≈ 90.909 USDC
```

**Notice:**
- Spot price = y₀/x₀ = 1,000/10 = 100 USDC/ETH
- Alice gets ≈90.909 USDC per ETH
- This is **worse** than spot price → **slippage**

### Adding Uniswap V2 Fee (0.3%)

Uniswap V2 takes a 0.3% fee from the input token.

**With fee:**
```
amountInWithFee = dx * 0.997 (keeping 99.7%)
dy = (amountInWithFee * y₀) / (x₀ + amountInWithFee)
```

**Or in Solidity (using integers):**
```solidity
amountInWithFee = dx * 997;
dy = (amountInWithFee * y₀) / (x₀ * 1000 + amountInWithFee);
```

**Example with fee:**
```
dx = 1 ETH
amountInWithFee = 1 * 0.997 = 0.997
dy = (0.997 * 1,000) / (10 + 0.997)
dy ≈ 90.661 USDC
```

### Slippage

**Spot price** = instantaneous pool price = `y₀ / x₀`

**Executed price** = actual price you get = `dy / dx`

**Slippage** = difference between spot and executed price

Slippage grows as trade size increases relative to pool reserves.

---

## Advanced Topics

### How External Market Affects AMM Price

**Key principle:**  
AMMs do NOT set the real price. The global market (Binance, Coinbase, etc.) sets the price.

**The process:**

1. **External market changes price**
   - Example: Whale buys 10,000 ETH on Binance
   - ETH price jumps from 100 → 200 USDC
   - This happens OUTSIDE Uniswap

2. **Uniswap still has old price**
   - Pool: 10 ETH | 1,000 USDC
   - Internal price = 100 USDC/ETH
   - But real market price is now 200

3. **Arbitrage traders notice**
   - Buy cheap ETH from Uniswap (100)
   - Sell expensive ETH on Coinbase (200)
   - Profit = 100 USDC per ETH

4. **Arbitrage changes pool**
   - Traders remove ETH, add USDC
   - Pool becomes: ~7.071 ETH | ~1,414 USDC
   - New internal price ≈ 200 USDC/ETH

5. **Equilibrium restored**
   - Uniswap price now matches market price
   - No more arbitrage opportunity

### Multi-Hop Swaps

Uniswap V2 Router can swap through multiple pools:

```
User wants: ETH → MKR
Path: ETH → USDC → MKR
Pools used: ETH/USDC, USDC/MKR
```

The router calls `getAmountsOut` iteratively for each hop.

---

## Quick Reference

### Key Formulas

| Formula | Purpose |
|---------|---------|
| `x * y = k` | CPMM invariant |
| `dy = (y₀ * dx) / (x₀ + dx)` | Output amount (no fee) |
| `price = y / x` | Spot price |
| `IL = 1 - (2√R / (R+1))` | Impermanent loss |

### Resources

- **Uniswap V2 Docs:** [docs.uniswap.org](https://docs.uniswap.org)
- **Cyfrin Updraft Course:** [updraft.cyfrin.io](https://updraft.cyfrin.io/courses/uniswap-v2)
- **Atrium Academy:** [learn.atrium.academy](https://learn.atrium.academy)

---

**Last Updated:** December 4, 2025
