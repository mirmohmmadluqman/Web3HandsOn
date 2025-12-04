# Lesson 1: Swap Math - SUPER SHORT VERSION

## What Are We Learning?
**Goal:** Find the formula to calculate how many tokens you GET when you GIVE tokens to Uniswap.

---

## The Symbols (Notation)

| Symbol | Meaning |
|--------|---------|
| `x₀` | Starting amount of token X in pool (the ₀ means "starting") |
| `y₀` | Starting amount of token Y in pool |
| `dx` | How much X Alice PUTS IN |
| `dy` | How much Y Alice GETS OUT (this is what we want to find) |

---

## The Example

**Pool starts with:**
- `x₀ = 10 ETH`
- `y₀ = 1,000 USDC`

**Alice wants to swap:**
- She gives: `dx = 1 ETH`
- She gets: `dy = ??? USDC` ← We need to calculate this

**After swap, pool will have:**
- `x₀ + dx = 11 ETH` (more ETH)
- `y₀ - dy = ???` (less USDC)

---

## The Magic Rule (Invariant)

Uniswap has ONE rule that NEVER changes:

```
x * y = k (constant)
```

This must be true BEFORE and AFTER the swap.

**Before swap:**
```
x₀ * y₀ = k
```

**After swap:**
```
(x₀ + dx) * (y₀ - dy) = k
```

Both equal `k`, so they equal each other!

---

## The Math (3 Steps)

### Step 1: Set them equal
```
x₀ * y₀ = (x₀ + dx) * (y₀ - dy)
```

### Step 2: Rearrange to isolate dy
Divide both sides by `(x₀ + dx)`:
```
y₀ - dy = (x₀ * y₀) / (x₀ + dx)
```

### Step 3: Solve for dy
```
dy = y₀ - (x₀ * y₀) / (x₀ + dx)
```

Simplify this and you get:

```
dy = (y₀ * dx) / (x₀ + dx)
```

---

## The Final Formula

```
dy = (y₀ * dx) / (x₀ + dx)
```

**In plain English:**
> Tokens OUT = (Pool Y × Input X) ÷ (Pool X + Input X)

---

## Real Example with Numbers

Pool: 10 ETH, 1000 USDC  
Alice gives: 1 ETH

```
dy = (1,000 * 1) / (10 + 1)
dy = 1,000 / 11
dy = 90.909 USDC
```

Alice gets **90.909 USDC** for her 1 ETH.

**Why not 100 USDC?**  
Because of **slippage** - bigger trades get worse prices.

---

## Important Notes

✅ This formula has **NO FEES**  
✅ Next lesson adds the 0.3% Uniswap fee  
✅ The bigger the `dx`, the worse the price (slippage)

---

## Questions to Ask Yourself

1. What does `x₀` mean? (Answer: starting amount of token X)
2. What does `dx` mean? (Answer: input amount Alice gives)
3. What does `dy` mean? (Answer: output amount Alice gets)
4. Why does the formula work? (Answer: because `x * y = k` must stay constant)
5. Why is the price worse than spot? (Answer: slippage from changing pool ratio)

---

**Now go ahead and ask me questions in the chat!** 💬
