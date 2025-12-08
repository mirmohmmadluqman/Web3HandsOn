# Complete Guide: Building Deposit & Withdraw Functions

## Your Homework Task

Burhan said:
- Create `deposit()` function - triggered when instruction_data[0] == **0**
- Create `withdraw()` function - triggered when instruction_data[0] == **1**
- Use **match** to distinguish between them
- Add **Solana transfer action** to move SOL

---

## Part 1: The Problem - How Does the Program Know What to Do?

Your program receives **raw bytes** from users. But how does it know if the user wants to:
- Deposit SOL? or
- Withdraw SOL?

**Answer:** The FIRST byte tells us!

---

## Part 2: The 0/1 System - How It Works

### When User Wants to DEPOSIT:
```
User sends: [0, ...other data...]
             ^
             First byte = 0 means "I want to deposit"
```

### When User Wants to WITHDRAW:
```
User sends: [1, ...other data...]
             ^
             First byte = 1 means "I want to withdraw"
```

### Visual Example:

```
User clicks "Deposit 50 SOL" button
    ↓
Frontend creates: [0, 50, 0, 0, 0, 0, 0, 0]
                   ^  ^--- amount (50)
                   |
                   0 = deposit instruction
    ↓
Sends to your program
    ↓
Your program reads first byte: instruction_data[0] = 0
    ↓
0 means deposit → calls deposit() function
```

```
User clicks "Withdraw 30 SOL" button
    ↓
Frontend creates: [1, 30, 0, 0, 0, 0, 0, 0]
                   ^  ^--- amount (30)
                   |
                   1 = withdraw instruction
    ↓
Sends to your program
    ↓
Your program reads first byte: instruction_data[0] = 1
    ↓
1 means withdraw → calls withdraw() function
```

---

## Part 3: Using MATCH to Distinguish

Rust's `match` is like a switch statement. It checks a value and runs different code:

### Simple Example First:
```rust
let number = 0;

match number {
    0 => println!("This is zero!"),
    1 => println!("This is one!"),
    _ => println!("This is something else!"),
}
// Output: "This is zero!"
```

### Now With instruction_data:
```rust
let first_byte = instruction_data[0];  // Get first byte

match first_byte {
    0 => {
        // User sent 0, so they want to DEPOSIT
        msg!("User wants to deposit");
        deposit(...)
    }
    1 => {
        // User sent 1, so they want to WITHDRAW
        msg!("User wants to withdraw");
        withdraw(...)
    }
    _ => {
        // User sent something else (2, 3, 4, etc.) - ERROR!
        msg!("Unknown instruction!");
        Err(ProgramError::InvalidInstructionData)
    }
}
```

### What `_` Means:
The underscore `_` is the **default case**. If the first byte is NOT 0 or 1, this catches everything else.

---

## Part 4: The Complete Flow

```
┌─────────────────────────────────────────────────────────────┐
│                        USER                                 │
│  Sends instruction_data = [0, 50, 0, 0, 0, 0, 0, 0]        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  process_instruction()                       │
│                                                              │
│  1. Check if instruction_data is empty                       │
│  2. Read first byte: instruction_data[0]                     │
│  3. Use MATCH to route:                                      │
│                                                              │
│     match instruction_data[0] {                              │
│         0 => deposit()    ← FIRST BYTE IS 0? GO HERE        │
│         1 => withdraw()   ← FIRST BYTE IS 1? GO HERE        │
│         _ => Error        ← ANYTHING ELSE? ERROR            │
│     }                                                        │
└─────────────────────────────────────────────────────────────┘
                              │
                    (first byte was 0)
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      deposit()                               │
│                                                              │
│  - Transfer SOL from User → Vault PDA                       │
│  - Uses invoke() because user signs                         │
└─────────────────────────────────────────────────────────────┘
```

---

## Part 5: Complete Boilerplate Code

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Set the entry point
entrypoint!(process_instruction);

// Main entry point - ALL calls come here first
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],       // <-- The raw bytes user sends
) -> ProgramResult {
    
    // Step 1: Check if empty
    if instruction_data.is_empty() {
        msg!("Error: No instruction data!");
        return Err(ProgramError::InvalidInstructionData);
    }

    // Step 2: Read first byte and route with MATCH
    match instruction_data[0] {
        0 => {
            msg!("Instruction: Deposit (first byte was 0)");
            deposit(program_id, accounts, &instruction_data[1..])
        }
        1 => {
            msg!("Instruction: Withdraw (first byte was 1)");
            withdraw(program_id, accounts, &instruction_data[1..])
        }
        _ => {
            msg!("Error: Unknown instruction, first byte was not 0 or 1");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

// Called when instruction_data[0] == 0
fn deposit(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],      // This is instruction_data[1..] (rest of data)
) -> ProgramResult {
    msg!("Deposit function running!");
    
    // TODO: Later add transfer logic here
    // solana_program::system_instruction::transfer(...)
    // invoke(...)
    
    Ok(())
}

// Called when instruction_data[0] == 1
fn withdraw(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],      // This is instruction_data[1..] (rest of data)
) -> ProgramResult {
    msg!("Withdraw function running!");
    
    // TODO: Later add transfer logic here
    // solana_program::system_instruction::transfer(...)
    // invoke_signed(...) - because vault PDA needs to sign
    
    Ok(())
}
```

---

## Part 6: Why This Works

| User Sends | First Byte | Match Result | Function Called |
|------------|------------|--------------|-----------------|
| `[0, 50, 0, 0, 0]` | 0 | matches `0 =>` | `deposit()` |
| `[1, 30, 0, 0, 0]` | 1 | matches `1 =>` | `withdraw()` |
| `[5, 10, 0, 0, 0]` | 5 | matches `_ =>` | Error! |
| `[]` (empty) | none | caught before match | Error! |

---

## Part 7: The Transfer Function (For Later)

When you actually move SOL:

### Deposit (User → Vault):
```rust
use solana_program::{program::invoke, system_instruction};

let transfer = system_instruction::transfer(
    user_pubkey,     // FROM: user
    vault_pubkey,    // TO: vault
    amount,          // HOW MUCH
);

invoke(&transfer, &[user_account, vault_account])?;
```

### Withdraw (Vault → User):
```rust
use solana_program::{program::invoke_signed, system_instruction};

let transfer = system_instruction::transfer(
    vault_pubkey,    // FROM: vault (PDA)
    user_pubkey,     // TO: user
    amount,          // HOW MUCH
);

// PDA can't sign normally, so we use invoke_signed with seeds
invoke_signed(
    &transfer, 
    &[vault_account, user_account],
    &[&[b"vault", &[bump_seed]]],  // The PDA seeds
)?;
```

---

## Summary

1. **User sends bytes** → `[0, ...]` or `[1, ...]`
2. **First byte** tells us what they want:
   - `0` = deposit
   - `1` = withdraw
3. **match** checks the first byte and routes to correct function
4. **deposit()** or **withdraw()** handles the actual logic

**That's it! The 0/1 system with match!** 🎉

---

## Test Yourself

1. User sends `[0, 100, 0, 0]`. What function runs?
2. User sends `[1, 50, 0, 0]`. What function runs?  
3. User sends `[3, 0, 0, 0]`. What happens?
4. Why do we use `instruction_data[1..]` when calling deposit/withdraw?
