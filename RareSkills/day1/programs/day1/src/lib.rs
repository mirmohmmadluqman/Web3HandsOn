use anchor_lang::prelude::*;

declare_id!("GA7CGitMusYA4y4uUPgRKLbHkRxhjtHXmio6bAJYufMR");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initializesnothing(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!"); // **** NEW LINE HERE ****
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
