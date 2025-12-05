# Learning: Pattern Matching & Solana Instruction Routing

## 📚 Step 1: Read This First
**Programiz Pattern Matching Tutorial:**  
https://www.programiz.com/rust/pattern-matching

---

## 🧠 What is Pattern Matching?

Pattern matching lets you check a value and run different code based on what it is.

```rust
let instruction = 0;

match instruction {
    0 => println!("This is deposit"),
    1 => println!("This is withdraw"),
    _ => println!("Unknown instruction"),
}
```

**Output:** `This is deposit`

### Key Points:
- `match` checks the value
- `0 =>` means "if value is 0, do this"
- `1 =>` means "if value is 1, do this"  
- `_ =>` means "anything else" (default case)

---

## 📚 Step 2: Study the Counter Example

**GitHub Link:**  
https://github.com/solana-developers/program-examples/blob/main/basics/counter/native/program/src/lib.rs

### What That Code Does:

```rust
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],  // ← Data comes as bytes
) -> ProgramResult {
    
    // Split: first byte = instruction type, rest = data
    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
    
    // Check what instruction it is
    match instruction_discriminant[0] {
        0 => {
            msg!("Instruction: Increment");
            process_increment_counter(accounts, instruction_data_inner)?;
        }
        _ => {
            msg!("Error: unknown instruction")
        }
    }
    Ok(())
}
```

### Breaking It Down:

| Part | What It Does |
|------|-------------|
| `instruction_data: &[u8]` | Raw bytes sent to the program |
| `split_at(1)` | Splits bytes: first byte vs rest |
| `instruction_discriminant[0]` | The first byte (0, 1, 2, etc.) |
| `match` | Checks what instruction to run |

---

## 🎯 Step 3: Your Task

Create TWO functions in `lib.rs`:
- **deposit** → triggered when first byte is `0`
- **withdraw** → triggered when first byte is `1`

### The Structure:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Entry point
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    // Check if we have data
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Route based on first byte
    match instruction_data[0] {
        0 => deposit(program_id, accounts, &instruction_data[1..]),
        1 => withdraw(program_id, accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Deposit function (placeholder)
fn deposit(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Deposit instruction called");
    Ok(())
}

// Withdraw function (placeholder)
fn withdraw(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Withdraw instruction called");
    Ok(())
}
```

---

## 🔑 Key Concepts Explained

### 1. `instruction_data[0]`
The **first byte** tells the program WHAT to do:
- `0` = deposit
- `1` = withdraw

### 2. `&instruction_data[1..]`
The **rest of the bytes** contain the actual data (amount, etc.)

### 3. Why `match`?
Because Rust forces you to handle ALL possible cases. You must handle 0, 1, AND everything else (`_`).

---

## ❓ Questions to Ask Yourself

1. What happens if `instruction_data` is empty?
2. Why do we use `&instruction_data[1..]` when calling deposit/withdraw?
3. What does `_` mean in the match expression?
4. Why do functions return `ProgramResult`?

---

## 📝 Next Steps

1. ✅ Read the Programiz article
2. ✅ Study the Counter example on GitHub
3. ✅ Try to write the code yourself
4. ❓ Ask questions here if stuck!
