## https://github.com/solana-developers/program-examples/blob/main/basics/counter/native/program/src/lib.rs :

```Rust
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    declare_id,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

mod state;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
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

pub fn process_increment_counter(
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let counter_account = next_account_info(account_info_iter)?;
    assert!(
        counter_account.is_writable,
        "Counter account must be writable"
    );

    let mut counter = Counter::try_from_slice(&counter_account.try_borrow_mut_data()?)?;
    counter.count += 1;
    counter.serialize(&mut *counter_account.data.borrow_mut())?;

    msg!("Counter state incremented to {:?}", counter.count);
    Ok(())
}
```
---
Read this lib.rs
```rust
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_data[0] {
        0 => deposit(program_id, accounts, &instruction_data[1..]),
        1 => withdraw(program_id, accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn deposit(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Deposit instruction");
    Ok(())
}

fn withdraw(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Withdraw instruction");
    Ok(())
}

#[allow(dead_code)]
#[allow(unused)]
fn main() {
    let my_option: Option<i32> = Some(222);

    // use of if let expression on the Option type
    if let Some(value) = my_option {
        println!("The option has a value of {}", value);
    } else {
        println!("The option has no value");
    }
}
```

---
# SAVED:
---

When you run:

```
solana program deploy target/deploy/myprogram.so
```

Solana says:

> “Okay, this .so file claims to be program X.
> Is this .so signed by the private key of program X?”

If yes → deployment succeeds
If no → **deployment fails**

## Rust Practice...

```Rust
// fn main() {
//     let instruction = 0;

//     match instruction {
//         0 => println!("This is deposit"),
//         1 => println!("This is withdraw"),
//         _ => println!("Unknown"),  // default case
//     }
// }

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    declare_id,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

mod state;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[cfg(not(feature = "no-entrypoint"))] // This means:
                                       //   “Compile this line ONLY if the feature no-entrypoint is NOT enabled.”
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))] // This means:
                                       //   “Compile this line ONLY if the feature no-entrypoint is NOT enabled.”
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
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

pub fn process_increment_counter(
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let counter_account = next_account_info(account_info_iter)?;
    assert!(
        counter_account.is_writable,
        "Counter account must be writable"
    );

    let mut counter = Counter::try_from_slice(&counter_account.try_borrow_mut_data()?)?;
    counter.count += 1;
    counter.serialize(&mut *counter_account.data.borrow_mut())?;

    msg!("Counter state incremented to {:?}", counter.count);
    Ok(())
}
```