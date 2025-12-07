// // fn main() {
// //     let instruction = 0;

// //     match instruction {
// //         0 => println!("This is deposit"),
// //         1 => println!("This is withdraw"),
// //         _ => println!("Unknown"),  // default case
// //     }
// // }

// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     declare_id,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     pubkey::Pubkey,
// };

// mod state;
// pub use state::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[cfg(not(feature = "no-entrypoint"))] // This means:
//                                        //   “Compile this line ONLY if the feature no-entrypoint is NOT enabled.”
// use solana_program::entrypoint;

// #[cfg(not(feature = "no-entrypoint"))] // This means:
//                                        //   “Compile this line ONLY if the feature no-entrypoint is NOT enabled.”
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
//     match instruction_discriminant[0] {
//         0 => {
//             msg!("Instruction: Increment");
//             process_increment_counter(accounts, instruction_data_inner)?;
//         }
//         _ => {
//             msg!("Error: unknown instruction")
//         }
//     }
//     Ok(())
// }

// pub fn process_increment_counter(
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> Result<(), ProgramError> {
//     let account_info_iter = &mut accounts.iter();

//     let counter_account = next_account_info(account_info_iter)?;
//     assert!(
//         counter_account.is_writable,
//         "Counter account must be writable"
//     );

//     let mut counter = Counter::try_from_slice(&counter_account.try_borrow_mut_data()?)?;
//     counter.count += 1;
fn deposit() {
    println!("Deposit");
}

fn withdraw() {
    println!("Withdraw");
}

fn main() {
    let instruction_data = [0];
    process_instruction(&instruction_data);
}
