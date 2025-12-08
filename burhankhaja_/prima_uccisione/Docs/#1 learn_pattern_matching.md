# Learning: Pattern Matching & Solana Instruction Routing

## 📚 Step 1: Read First
**Programiz Pattern Matching:**  
https://www.programiz.com/rust/pattern-matching

---

## 🧠 What is Pattern Matching?

```rust
let instruction = 0;

match instruction {
    0 => println!("This is deposit"),
    1 => println!("This is withdraw"),
    _ => println!("Unknown"),  // default case
}
```

**Output:** `This is deposit`

---

## 📚 Step 2: Study Counter Example
**GitHub Link:**  
https://github.com/solana-developers/program-examples/blob/main/basics/counter/native/program/src/lib.rs

### Key Code:
```rust
let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);

match instruction_discriminant[0] {
    0 => process_increment_counter(accounts, instruction_data_inner)?,
    _ => msg!("Error: unknown instruction"),
}
```

---

## 🎯 Your Task Structure

```rust
match instruction_data[0] {
    0 => deposit(...),    // first byte = 0
    1 => withdraw(...),   // first byte = 1
    _ => Err(ProgramError::InvalidInstructionData),
}
```

---

## 🔑 Key Concepts

| Concept | Meaning |
|---------|---------|
| `instruction_data[0]` | First byte = instruction type |
| `&instruction_data[1..]` | Rest of bytes = actual data |
| `_ =>` | Default case (anything else) |

---

## ❓ Questions to Ask Yourself

1. What happens if `instruction_data` is empty?
2. Why do we pass `&instruction_data[1..]` to functions?
3. What does `_` mean in match?

**Read, study, then ask me questions!**
