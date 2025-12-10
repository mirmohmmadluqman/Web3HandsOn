# Understanding the Prima Uccisione Architecture

## Overview

This document explains the architectural diagram for the **Prima Uccisione** Solana program - a simple deposit/withdraw contract.

---

## The Flow

```
User → sends instruction_data → process_instruction() → deposit() or withdraw() → transfer → VAULT PDA
```

---

## Components

### 1. **User**
- Has a `Pubkey` (wallet address)
- Sends `sol_amount: u64` (amount to deposit/withdraw)

### 2. **process_instruction()**
The entry point. Uses pattern matching:

```rust
match instruction_data[0] {
    0 => deposit(),    // first byte = 0
    1 => withdraw(),   // first byte = 1
    _ => Error
}
```

### 3. **deposit()**
- Triggered when `instruction_data[0] == 0`
- Uses `invoke()` to call transfer
- Moves SOL from User → VAULT PDA

### 4. **withdraw()**
- Triggered when `instruction_data[0] == 1`
- Uses `invoke_signed()` (PDA needs to sign!)
- Moves SOL from VAULT PDA → User

### 5. **solana_program::system_instruction::transfer**
The actual Solana function that moves SOL between accounts.

- **deposit uses:** `invoke()` - user signs
- **withdraw uses:** `invoke_signed()` - PDA signs with seeds

### 6. **PROGRAM VAULT PDA**
- Program Derived Address
- Holds all deposited SOL
- Only the program can withdraw from it

---

## Key Concepts

### What is `instruction_data`?
Raw bytes sent to the program. The first byte tells us WHAT to do:
- `[0, ...]` → deposit
- `[1, ...]` → withdraw

### What is `invoke()` vs `invoke_signed()`?

| Function | Who Signs | When to Use |
|----------|-----------|-------------|
| `invoke()` | User | deposit (user sends their SOL) |
| `invoke_signed()` | PDA | withdraw (program sends from vault) |

### What is a PDA?
Program Derived Address - an address controlled by the program, not a wallet.

---

## Burhan's Notes

- First byte of instruction_data tells the action (0=deposit, 1=withdraw)
- Use `solana_program::system_instruction::transfer` for moving SOL
- PDA must use `invoke_signed()` because it has no private key

---

## Questions to Ask Yourself

1. Why does deposit use `invoke()` but withdraw uses `invoke_signed()`?
2. How does the program know which instruction to run?
3. What happens if instruction_data[0] is not 0 or 1?
