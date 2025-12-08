use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction_datax = instruction_data;

    if instruction_datax.len() < 1 {
        // or we can write if instruction_datax.is_empty()... I read this on #4 DOC
        msg!("No instruction provided");
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_datax[0] {
        0 => deposit(program_id, accounts, &instruction_datax[1..]),
        1 => withdraw(program_id, accounts, &instruction_datax[1..]),
        _ => Err(ProgramError::InvalidInstructionData)
        // _ => {
        //     msg!("Unknown instruction");
        //     Ok(())
        // }
    }
}

fn deposit(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Deposit");
    // Functionality:
    // 1. Get the accounts passed in. Order Matters.
    let accounts_iter = &mut _accounts.iter();               // _accounts.iter() creates the iterator
    let user_account = next_account_info(_accounts.iter())?; // next_account_info(...) pulls the actual account out of the iterator
    let vault_account = next_account_info(_accounts.iter())?;
    // The ammount would be parsed from instruction_data
    let amount = u64::from_le_bytes(_instruction_data.try_into().unwrap()); 

    Ok(())
}

fn withdraw(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Withdraw");
    // TODO...
    Ok(())
}

// fn deposit() -> ProgramResult {
//     msg!("Deposit");
//     Ok(())
// }

// fn withdraw() -> ProgramResult {
//     msg!("Withdraw");
//     Ok(())
// }
