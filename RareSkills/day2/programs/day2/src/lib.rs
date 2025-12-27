use anchor_lang::prelude::*;

declare_id!("8GLs8BBRi6hKC1N7JxjvSCagLQDzwEPggomihBB1jpGW");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
