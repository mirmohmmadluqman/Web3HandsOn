# Solana Architecture: The Mental Model Guide

This guide explains **WHY** things work the way they do in Solana, specifically for your Deposit/Withdraw program.

## 1. The Big Picture: The Robot Banker Analogy

Imagine your Solana Program is a **Robot Banker** inside a giant secure building (The Blockchain).

*   **Accounts**: These are **Safe Deposit Boxes** on the wall. Some hold Data (paper), some hold SOL (gold coins).
*   **The User (You)**: You have a key to your own box.
*   **The Program (Robot)**: It has no hands, but it can instruct the building security (System Program) to move things.

---

## 2. The Core Concepts

### A. Where does the SOL live?
SOL *never* "travels" through the air. It always sits inside an **Account**.
*   **Your Wallet**: An account owned by the System Program, but *you* have the key (Private Key) to authorize changes.
*   **The Vault**: An account owned by the System Program, but *controlled* by your Program (PDA).

### B. What is a PDA (Program Derived Address)?
*   **Normal Account**: Has a Public Key (Address) and a Private Key (Password). You need the password to move SOL out.
*   **PDA**: Has a Public Key, but **NO Private Key**. It is impossible for a human to sign for it.
    *   *Why?* Because we want the **Program** to be the only thing that can move funds from the Vault. If there was a private key, the developer could steal the money!
    *   *How it works:* The Blockchain knows: "If the **Prima Program** says 'Move money from this PDA', I will allow it."

---

## 3. The Two Flows: Deposit vs. Withdraw

### Flow 1: Deposit (The "Invoke" Flow)
**Goal:** Move SOL from **User** -> **Vault**.

1.  **You (User)** create a transaction: "Move 5 SOL from Me to Vault".
2.  **You Sign** the transaction with your Private Key.
3.  **The Program** runs `process_deposit`:
    *   It sees you want to move money.
    *   It needs to tell the **System Program** (the building security) to actually move the coins.
    *   **CRITICAL:** The System Program will ask: "Does the owner of the 'From' account agree?"
    *   **Answer:** YES! You signed the transaction at the start.
    *   **Code:** We use `invoke(...)`. This passes *your* signature along to the System Program.

> **Mental Check:** `invoke` = "Pass along the user's existing signature."

### Flow 2: Withdraw (The "Invoke_Signed" Flow)
**Goal:** Move SOL from **Vault** -> **User**.

1.  **You (User)** create a transaction: "Withdraw 5 SOL from Vault to Me".
2.  **You Sign** it (to prove you are the one asking).
3.  **The Program** runs `process_withdraw`:
    *   It checks your balance in the State (Green Box). "Does he have 5 SOL?" -> Yes.
    *   Now it needs to move SOL from **Vault** -> **User**.
    *   It calls the **System Program**.
    *   **CRITICAL:** The System Program asks: "Does the owner of the 'From' account (The Vault) agree?"
    *   **Problem:** The Vault is a PDA. It has no private key. No human signed for it.
    *   **Solution:** The Program says: "I AM the owner of this Vault. I authorize this."
    *   **Code:** We use `invoke_signed(...)`. This tells the blockchain: "Trust me, I am the program, and I am 'virtually signing' for this PDA."

> **Mental Check:** `invoke_signed` = "The Program creates a signature on the fly for its PDA."

---

## 4. The Code Breakdown

### `accounts: &[AccountInfo]`
Think of this as a **Bag of Keys**. When you call the program, you must pass in *every single account* you plan to touch.
*   `accounts[0]`: The User (Signer)
*   `accounts[1]`: The Vault (PDA)
*   `accounts[2]`: The System Program (The Mover)

### `invoke` (Deposit)
```rust
invoke(
    &system_instruction::transfer(user.key, vault.key, amount), // The Instruction
    &[user.clone(), vault.clone()],                             // The Accounts involved
)
```
*   **Why?** The `user` account is the one losing money. The `user` signed the transaction. `invoke` uses that signature.

### `invoke_signed` (Withdraw)
```rust
invoke_signed(
    &system_instruction::transfer(vault.key, user.key, amount), // The Instruction
    &[vault.clone(), user.clone()],                             // The Accounts involved
    &[&[b"vault", &[bump_seed]]],                               // The "Virtual Signature" (Seeds)
)
```
*   **Why?** The `vault` is losing money. It cannot sign.
*   **The Seeds:** `&[b"vault", ...]` proves that this PDA actually belongs to *this* program. The blockchain checks: "Does `hash("vault", program_id)` equal this address?" If yes, it allows the transfer.

---

## 5. Summary Table

| Feature | Deposit | Withdraw |
| :--- | :--- | :--- |
| **Direction** | User -> Vault | Vault -> User |
| **Who loses money?** | User | Vault (PDA) |
| **Who must sign?** | User (Real Key) | Program (Virtual Key) |
| **Function** | `invoke()` | `invoke_signed()` |
| **Why?** | User signed the Tx | PDA cannot sign; Program signs for it |

---

## 6. Your Homework (The Withdraw Diagram)

Now, look at your diagram again:
1.  **Arrow User -> Withdraw**: You calling the function.
2.  **Arrow Withdraw -> State**: Checking your balance.
3.  **Arrow Vault -> User**: This is the `invoke_signed` transfer!

You are now ready to explain this to your teacher!
